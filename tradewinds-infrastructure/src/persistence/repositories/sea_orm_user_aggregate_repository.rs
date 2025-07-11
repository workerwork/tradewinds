use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionError,
    TransactionTrait,
};
use tokio::time::sleep;
use uuid::Uuid;

use crate::persistence::entities::{user, user_role};
use tradewinds_domain::value_objects::auth::{AuthUsername, Password};
use tradewinds_domain::value_objects::user::{Avatar, Email, Phone, RealName, UserStatus};
use tradewinds_domain::{
    aggregates::user_aggregate::UserAggregate,
    entities::{user::User, user_role::UserRole},
    repositories::user_aggregate_repository::UserAggregateRepository,
    value_objects::{role::RoleId, user::UserId},
};
use tradewinds_error::{AppError, AppResult};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 100;

#[derive(Debug, Clone)]
pub struct SeaOrmUserAggregateRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserAggregateRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn user_from_model(&self, model: user::Model) -> AppResult<User> {
        Ok(User {
            id: UserId::new(model.id)?,
            username: AuthUsername::new(model.username)?,
            email: Email::new(model.email)?,
            password: Password::new(model.password)?,
            status: UserStatus::from_i32(model.status)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
            real_name: model.real_name.map(RealName::new).transpose()?,
            avatar: model.avatar.map(Avatar::new).transpose()?,
            phone: model.phone.map(Phone::new).transpose()?,
        })
    }

    fn user_to_active_model(&self, user_entity: &User) -> user::ActiveModel {
        let password = user_entity.password.value().to_string();
        let created_at = DateTime::from_timestamp(user_entity.created_at, 0).expect("Invalid timestamp").into();
        let updated_at = DateTime::from_timestamp(user_entity.updated_at, 0).expect("Invalid timestamp").into();

        user::ActiveModel {
            id: Set(user_entity.id.value().to_string()),
            username: Set(user_entity.username.value().to_string()),
            email: Set(user_entity.email.value().to_string()),
            password: Set(password),
            status: Set(user_entity.status.value()),
            real_name: Set(user_entity.real_name.as_ref().map(|r| r.value().to_string())),
            avatar: Set(user_entity.avatar.as_ref().map(|a| a.value().to_string())),
            phone: Set(user_entity.phone.as_ref().map(|p| p.value().to_string())),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
        }
    }

    async fn verify_user_insert<C>(&self, conn: &C, user_id: &str, retry_count: u32) -> Result<bool, DbErr>
    where
        C: ConnectionTrait,
    {
        for attempt in 0..retry_count {
            if attempt > 0 {
                sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
            }

            match user::Entity::find_by_id(user_id).one(conn).await? {
                Some(_) => return Ok(true),
                None if attempt == retry_count - 1 => return Ok(false),
                None => continue,
            }
        }
        Ok(false)
    }

    async fn verify_user_role_insert<C>(
        &self,
        conn: &C,
        user_id: &str,
        role_id: &str,
        retry_count: u32,
    ) -> Result<bool, DbErr>
    where
        C: ConnectionTrait,
    {
        for attempt in 0..retry_count {
            if attempt > 0 {
                sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
            }

            let result = user_role::Entity::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .filter(user_role::Column::RoleId.eq(role_id))
                .one(conn)
                .await?;

            match result {
                Some(_) => return Ok(true),
                None if attempt == retry_count - 1 => return Ok(false),
                None => continue,
            }
        }
        Ok(false)
    }
}

#[async_trait]
impl UserAggregateRepository for SeaOrmUserAggregateRepository {
    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<UserAggregate>> {
        let user_model = user::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find user by id: {}", e)))?;

        let Some(user_model) = user_model else {
            return Ok(None);
        };

        let role_ids = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(id.value()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find roles for user: {}", e)))?
            .into_iter()
            .map(|ur| RoleId::new(ur.role_id).unwrap())
            .collect::<Vec<_>>();

        let user_entity = self.user_from_model(user_model)?;
        let aggregate = UserAggregate { user: user_entity, roles: role_ids };
        Ok(Some(aggregate))
    }

    async fn create(&self, aggregate: &UserAggregate) -> AppResult<()> {
        let user_model = self.user_to_active_model(&aggregate.user);
        let role_count = aggregate.roles.len();
        let role_models = UserRole::create_associations(aggregate.user.id.clone(), &aggregate.roles);
        let repo = self.clone();

        self.db
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    // 插入用户
                    let user_id = user_model.id.clone().unwrap();
                    match user::Entity::insert(user_model).exec(txn).await {
                        Ok(_) => {
                            // 验证用户插入
                            if !repo.verify_user_insert(txn, &user_id, MAX_RETRIES).await? {
                                return Err(DbErr::Custom("Failed to verify user insertion after retries".to_string()));
                            }
                        }
                        Err(e) => {
                            // 即使插入报错，也尝试验证
                            if repo.verify_user_insert(txn, &user_id, MAX_RETRIES).await? {
                                // 记录已存在，继续处理
                            } else {
                                return Err(DbErr::Custom(format!("Failed to insert user: {}", e)));
                            }
                        }
                    }

                    // 插入用户角色
                    if !role_models.is_empty() {
                        for (i, (id, user_id, role_id)) in role_models.into_iter().enumerate() {
                            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
                            let role_model = user_role::ActiveModel {
                                id: Set(id),
                                user_id: Set(user_id.clone()),
                                role_id: Set(role_id.clone()),
                                created_at: Set(now),
                                updated_at: Set(now),
                            };

                            match user_role::Entity::insert(role_model).exec(txn).await {
                                Ok(_) => {
                                    // 验证角色关联插入
                                    if !repo.verify_user_role_insert(txn, &user_id, &role_id, MAX_RETRIES).await? {
                                        return Err(DbErr::Custom(format!(
                                            "Failed to verify user role {}/{} insertion after retries",
                                            i + 1,
                                            role_count
                                        )));
                                    }
                                }
                                Err(e) => {
                                    // 即使插入报错，也尝试验证
                                    if repo.verify_user_role_insert(txn, &user_id, &role_id, MAX_RETRIES).await? {
                                        // 记录已存在，继续处理
                                    } else {
                                        return Err(DbErr::Custom(format!(
                                            "Failed to insert user role {}/{}: {}",
                                            i + 1,
                                            role_count,
                                            e
                                        )));
                                    }
                                }
                            }
                        }
                    }

                    Ok(())
                })
            })
            .await
            .map_err(|e| AppError::DatabaseError(format!("Transaction failed: {}", e)))
    }

    async fn save(&self, aggregate: &UserAggregate) -> AppResult<()> {
        let repo = self.clone();
        let user_model = self.user_to_active_model(&aggregate.user);
        let user_id = aggregate.user.id.value().to_string();
        let role_models = UserRole::create_associations(aggregate.user.id.clone(), &aggregate.roles);

        self.db
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    // 更新用户
                    user::Entity::update(user_model)
                        .exec(txn)
                        .await
                        .map_err(|e| DbErr::Custom(format!("Failed to update user: {}", e)))?;

                    // 删除现有角色关联
                    user_role::Entity::delete_many()
                        .filter(user_role::Column::UserId.eq(&user_id))
                        .exec(txn)
                        .await
                        .map_err(|e| DbErr::Custom(format!("Failed to delete existing user roles: {}", e)))?;

                    // 插入新的角色关联
                    for (id, user_id, role_id) in role_models {
                        let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
                        let role_model = user_role::ActiveModel {
                            id: Set(id),
                            user_id: Set(user_id.clone()),
                            role_id: Set(role_id.clone()),
                            created_at: Set(now),
                            updated_at: Set(now),
                        };

                        match user_role::Entity::insert(role_model)
                            .exec(txn)
                            .await
                            .map_err(|e| DbErr::Custom(format!("Failed to insert user role: {}", e)))
                        {
                            Ok(_) => {
                                // 验证角色关联插入
                                if !repo.verify_user_role_insert(txn, &user_id, &role_id, MAX_RETRIES).await? {
                                    return Err(DbErr::Custom(format!(
                                        "Failed to verify user role insertion after retries"
                                    )));
                                }
                            }
                            Err(e) => {
                                // 即使插入报错，也尝试验证
                                if repo.verify_user_role_insert(txn, &user_id, &role_id, MAX_RETRIES).await? {
                                    // 记录已存在，继续处理
                                } else {
                                    return Err(DbErr::Custom(format!("Failed to insert user role: {}", e)));
                                }
                            }
                        }
                    }

                    Ok(())
                })
            })
            .await
            .map_err(|e| AppError::DatabaseError(format!("Transaction failed: {}", e)))
    }

    async fn delete_by_id(&self, id: &UserId) -> AppResult<()> {
        use crate::persistence::entities::user;
        use sea_orm::EntityTrait;
        user::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete user by id failed: {}", e)))?;
        Ok(())
    }
}
