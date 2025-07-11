use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
};

use crate::persistence::entities::{role, role_permission};
use tradewinds_domain::aggregates::role_aggregate::RoleAggregate;
use tradewinds_domain::entities::{role::Role, role_permission::RolePermission};
use tradewinds_domain::repositories::role_aggregate_repository::RoleAggregateRepository;
use tradewinds_domain::value_objects::{
    RoleDescription, RoleName, RoleStatus, permission::permission_id::PermissionId, role::RoleCode,
    role::role_id::RoleId, role_permission::role_permission_id::RolePermissionId,
};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct SeaOrmRoleAggregateRepository {
    db: DatabaseConnection,
}

impl SeaOrmRoleAggregateRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn role_from_model(&self, model: role::Model) -> AppResult<Role> {
        Ok(Role {
            id: RoleId::new(model.id)?,
            code: RoleCode::new(model.code)?,
            name: RoleName::new(model.name)?,
            description: model.description.map(RoleDescription::new).transpose()?,
            status: RoleStatus::from_i32(model.status)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn role_to_active_model(&self, role: &Role) -> role::ActiveModel {
        let now = chrono::Utc::now();
        role::ActiveModel {
            id: Set(role.id.value().to_string()),
            code: Set(role.code.value().to_string()),
            name: Set(role.name.value().to_string()),
            description: Set(role.description.as_ref().map(|d| d.value().to_string())),
            status: Set(role.status.value()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        }
    }

    fn role_permission_from_model(&self, model: role_permission::Model) -> AppResult<RolePermission> {
        Ok(RolePermission {
            id: RolePermissionId::new(model.id)?,
            role_id: RoleId::new(model.role_id)?,
            permission_id: PermissionId::new(model.permission_id)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn role_permission_to_active_model(&self, role_permission: &RolePermission) -> role_permission::ActiveModel {
        let now: DateTime<Utc> = Utc::now();
        role_permission::ActiveModel {
            id: Set(role_permission.id.value().to_string()),
            role_id: Set(role_permission.role_id.value().to_string()),
            permission_id: Set(role_permission.permission_id.value().to_string()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        }
    }
}

#[async_trait]
impl RoleAggregateRepository for SeaOrmRoleAggregateRepository {
    async fn find_by_id(&self, id: &RoleId) -> AppResult<Option<RoleAggregate>> {
        let role_model = role::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find role by id: {}", e)))?;

        let Some(role_model) = role_model else {
            return Ok(None);
        };

        let permission_ids = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(id.value()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find permissions for role: {}", e)))?
            .into_iter()
            .map(|rp| PermissionId::new(rp.permission_id))
            .collect::<AppResult<Vec<_>>>()?;

        let role_entity = self.role_from_model(role_model)?;

        let aggregate = RoleAggregate::from_existing(role_entity, permission_ids);

        Ok(Some(aggregate))
    }

    async fn create(&self, aggregate: &RoleAggregate) -> AppResult<()> {
        let role_model = self.role_to_active_model(&aggregate.role);
        let permission_models: Vec<role_permission::ActiveModel> = aggregate
            .permissions
            .iter()
            .map(|pid| role_permission::ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()), // 修复：为每条记录分配唯一id
                role_id: Set(aggregate.role.id.value().to_string()),
                permission_id: Set(pid.to_string()),
                ..Default::default()
            })
            .collect();
        let role_id = aggregate.role.id.value().to_string();
        let role_model_cloned = role_model.clone();
        let permission_models_cloned = permission_models.clone();
        self.db
            .transaction(move |txn| {
                let role_model = role_model_cloned.clone();
                let permission_models = permission_models_cloned.clone();
                let role_id = role_id.clone();
                Box::pin(async move {
                    // 角色插入幂等兼容
                    match role_model.insert(txn).await {
                        Ok(_) => {}
                        Err(e) => {
                            let exists = role::Entity::find_by_id(role_id.clone())
                                .one(txn)
                                .await
                                .map_err(|e| AppError::DatabaseError(format!("Failed to verify role insert: {}", e)))?
                                .is_some();
                            if !exists {
                                return Err(AppError::DatabaseError(format!("Failed to insert role: {}", e)));
                            }
                        }
                    }
                    // 权限关联插入（幂等兼容）
                    for permission_model in permission_models {
                        let role_id_val = permission_model.role_id.clone().unwrap();
                        let permission_id_val = permission_model.permission_id.clone().unwrap();
                        match role_permission::Entity::insert(permission_model).exec(txn).await {
                            Ok(_) => {
                                // 可选：查验插入
                            }
                            Err(e) => {
                                // 查验主键是否已存在
                                let exists = role_permission::Entity::find()
                                    .filter(role_permission::Column::RoleId.eq(role_id_val.clone()))
                                    .filter(role_permission::Column::PermissionId.eq(permission_id_val.clone()))
                                    .one(txn)
                                    .await
                                    .map_err(|e| {
                                        AppError::DatabaseError(format!(
                                            "Failed to verify role_permission insert: {}",
                                            e
                                        ))
                                    })?
                                    .is_some();
                                if !exists {
                                    return Err(AppError::DatabaseError(format!(
                                        "Failed to insert role_permission: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    }
                    Ok(())
                })
            })
            .await
            .map_err(|e: sea_orm::TransactionError<AppError>| {
                AppError::DatabaseError(format!("Failed to create role aggregate: {}", e))
            })
    }

    async fn save(&self, aggregate: &RoleAggregate) -> AppResult<()> {
        let role_model = self.role_to_active_model(&aggregate.role);

        let permission_models: Vec<role_permission::ActiveModel> = aggregate
            .permissions
            .iter()
            .map(|pid| role_permission::ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()),
                role_id: Set(aggregate.role.id.value().to_string()),
                permission_id: Set(pid.to_string()),
                ..Default::default()
            })
            .collect();

        let role_id = aggregate.role.id.value().to_string();
        let permission_models = permission_models.clone();
        self.db
            .transaction(|txn| {
                Box::pin(async move {
                    // 幂等兼容：只要update不报错或RecordNotUpdated就认为成功
                    use sea_orm::DbErr;
                    let update_result = role::Entity::update(role_model).exec(txn).await;
                    match update_result {
                        Ok(_) => {}
                        Err(DbErr::RecordNotUpdated) => {
                            // 幂等兼容：无记录被更新也算成功
                        }
                        Err(e) => return Err(AppError::DatabaseError(format!("Failed to update role: {}", e))),
                    }

                    // 验证角色是否确实存在，避免外键约束失败
                    let role_exists = role::Entity::find_by_id(role_id.clone()).one(txn).await?.is_some();
                    if !role_exists {
                        return Err(AppError::DatabaseError(format!("Role with id {} does not exist", role_id)));
                    }

                    role_permission::Entity::delete_many()
                        .filter(role_permission::Column::RoleId.eq(role_id.clone()))
                        .exec(txn)
                        .await?;

                    // 逐条插入新关联，幂等兼容
                    for permission_model in permission_models {
                        let role_id_val = permission_model.role_id.clone().unwrap();
                        let permission_id_val = permission_model.permission_id.clone().unwrap();
                        match role_permission::Entity::insert(permission_model).exec(txn).await {
                            Ok(_) => {}
                            Err(e) => {
                                // 即使插入报错，也尝试验证
                                if verify_role_permission_insert(txn, &role_id_val, &permission_id_val, 3).await? {
                                    // 记录已存在，继续处理
                                } else {
                                    return Err(AppError::DatabaseError(format!(
                                        "Failed to insert role_permission: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    }

                    Ok(())
                })
            })
            .await
            .map_err(|e: sea_orm::TransactionError<AppError>| {
                AppError::DatabaseError(format!("Failed to save role aggregate: {}", e))
            })
    }

    async fn delete_by_id(&self, id: &RoleId) -> AppResult<()> {
        use crate::persistence::entities::role;
        use sea_orm::EntityTrait;
        role::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete role by id failed: {}", e)))?;
        Ok(())
    }
}

// 新增辅助函数
async fn verify_role_permission_insert<C>(
    conn: &C,
    role_id: &str,
    permission_id: &str,
    retry_count: u32,
) -> Result<bool, sea_orm::DbErr>
where
    C: sea_orm::ConnectionTrait,
{
    for attempt in 0..retry_count {
        if attempt > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        let result = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .filter(role_permission::Column::PermissionId.eq(permission_id))
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
