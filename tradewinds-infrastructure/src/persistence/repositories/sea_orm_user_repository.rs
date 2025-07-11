use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseBackend, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, QueryTrait, Set,
};

use tradewinds_domain::entities::user::User;
use tradewinds_domain::repositories::UserRepository;
use tradewinds_domain::value_objects::user::{UserId, UserStatus};
use tradewinds_domain::value_objects::{
    Avatar, Password, Phone, RealName, auth::auth_username::AuthUsername, user::user_email::Email,
};

use crate::persistence::entities::user;
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn from_model(&self, model: user::Model) -> AppResult<User> {
        Ok(User {
            id: UserId::new(model.id)?,
            username: AuthUsername::new(model.username)?,
            email: Email::new(model.email)?,
            password: Password::new(model.password)?,
            real_name: model.real_name.map(RealName::new).transpose()?,
            phone: model.phone.map(Phone::new).transpose()?,
            avatar: model.avatar.map(Avatar::new).transpose()?,
            status: UserStatus::from_i32(model.status)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn to_active_model(&self, user: &User) -> user::ActiveModel {
        let now: DateTime<Utc> = Utc::now();
        user::ActiveModel {
            id: Set(user.id.value().to_string()),
            username: Set(user.username.value().to_string()),
            email: Set(user.email.value().to_string()),
            password: Set(user.password.value().to_string()),
            real_name: Set(user.real_name.as_ref().map(|v| v.value().to_string())),
            phone: Set(user.phone.as_ref().map(|v| v.value().to_string())),
            avatar: Set(user.avatar.as_ref().map(|v| v.value().to_string())),
            status: Set(user.status.value()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        }
    }
}

// 实现 UserRepository 接口，使用 SeaORM 作为数据库操作的实现
#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
        user::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find user by id failed: {}", e)))?
            .map(|model| self.from_model(model))
            .transpose()
    }

    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email.value()))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find user by email failed: {}", e)))?
            .map(|model| self.from_model(model))
            .transpose()
    }

    async fn find_by_username(&self, username: &AuthUsername) -> AppResult<Option<User>> {
        user::Entity::find()
            .filter(user::Column::Username.eq(username.value()))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find user by username failed: {}", e)))?
            .map(|model| self.from_model(model))
            .transpose()
    }

    async fn find_by_ids(&self, ids: &[UserId]) -> AppResult<Vec<User>> {
        let id_strs: Vec<String> = ids.iter().map(|id| id.value().to_string()).collect();
        user::Entity::find()
            .filter(user::Column::Id.is_in(id_strs))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find users by ids failed: {}", e)))?
            .into_iter()
            .map(|model| self.from_model(model))
            .collect()
    }

    async fn exists_by_username(&self, username: &AuthUsername) -> AppResult<bool> {
        let count = user::Entity::find()
            .filter(user::Column::Username.eq(username.value()))
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Check username exists failed: {}", e)))?;
        Ok(count > 0)
    }

    async fn exists_by_email(&self, email: &Email) -> AppResult<bool> {
        let count = user::Entity::find()
            .filter(user::Column::Email.eq(email.value()))
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Check email exists failed: {}", e)))?;
        Ok(count > 0)
    }

    async fn count(&self) -> AppResult<u64> {
        user::Entity::find().count(&self.db).await.map_err(|e| AppError::DatabaseError(format!("Count users failed: {}", e)))
    }

    async fn search(
        &self,
        username: Option<&AuthUsername>,
        phone: Option<&str>,
        email: Option<&Email>,
        status: Option<UserStatus>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<User>, u64)> {
        let mut query = user::Entity::find();
        if let Some(username) = username {
            query = query.filter(user::Column::Username.contains(username.value()));
        }
        if let Some(phone) = phone {
            query = query.filter(user::Column::Phone.contains(phone));
        }
        if let Some(email) = email {
            query = query.filter(user::Column::Email.contains(email.value()));
        }
        if let Some(status) = status {
            query = query.filter(user::Column::Status.eq(status.value()));
        } else if show_deleted == Some(true) {
            // 未传 status，且要求显示已删除用户，查所有
            // 不加 status 过滤
        } else {
            // 未传 status，且不显示已删除用户，只查未删除（0/1）
            query = query.filter(user::Column::Status.is_in([0, 1]));
        }
        let total = query
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Count users failed: {}", e)))?;
        let stmt = query.clone().build(DatabaseBackend::MySql);
        let models = query
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("List users failed: {}", e)))?;
        let users = models.into_iter().map(|model| self.from_model(model)).collect::<AppResult<Vec<User>>>()?;
        Ok((users, total))
    }
}
