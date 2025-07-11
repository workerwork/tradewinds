use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
};

use crate::persistence::entities::permission;
use tradewinds_domain::aggregates::permission_aggregate::PermissionAggregate;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_domain::repositories::permission_aggregate_repository::PermissionAggregateRepository;
use tradewinds_domain::value_objects::permission::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType,
};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct SeaOrmPermissionAggregateRepository {
    db: DatabaseConnection,
}

impl SeaOrmPermissionAggregateRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn from_model(&self, model: permission::Model) -> AppResult<Permission> {
        Ok(Permission {
            id: PermissionId::new(model.id)?,
            name: PermissionName::new(model.name)?,
            code: model.code.map(PermissionCode::new).transpose()?,
            type_: PermissionType::from_i32(model.type_)?,
            parent_id: model.parent_id.map(PermissionId::new).transpose()?,
            path: model.path.map(PermissionPath::new).transpose()?,
            component: model.component.map(PermissionComponent::new).transpose()?,
            icon: model.icon.map(PermissionIcon::new).transpose()?,
            sort: PermissionSort::new(model.sort)?,
            status: PermissionStatus::from_i32(model.status)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn to_active_model(&self, permission: &Permission) -> permission::ActiveModel {
        let now: DateTime<Utc> = Utc::now();
        permission::ActiveModel {
            id: Set(permission.id.value().to_string()),
            name: Set(permission.name.value().to_string()),
            code: Set(permission.code.as_ref().map(|c| c.value().to_string())),
            type_: Set(permission.type_.value()),
            parent_id: Set(permission.parent_id.as_ref().map(|p| p.value().to_string())),
            path: Set(permission.path.as_ref().map(|p| p.value().to_string())),
            component: Set(permission.component.as_ref().map(|c| c.value().to_string())),
            icon: Set(permission.icon.as_ref().map(|i| i.value().to_string())),
            sort: Set(permission.sort.value()),
            status: Set(permission.status.value()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        }
    }
}

#[async_trait]
impl PermissionAggregateRepository for SeaOrmPermissionAggregateRepository {
    async fn find_by_id(&self, id: &PermissionId) -> AppResult<Option<PermissionAggregate>> {
        let model = permission::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find permission by id: {}", e)))?;

        let Some(model) = model else {
            return Ok(None);
        };

        let entity = self.from_model(model)?;
        Ok(Some(PermissionAggregate { permission: entity }))
    }

    async fn create(&self, aggregate: &PermissionAggregate) -> AppResult<()> {
        let model = self.to_active_model(&aggregate.permission);
        let permission_id = aggregate.permission.id.value().to_string();
        let code = aggregate.permission.code.as_ref().map(|c| c.value().to_string());
        match model.insert(&self.db).await {
            Ok(_) => Ok(()),
            Err(e) => {
                // 插入报错时查验是否已存在
                let exists = permission::Entity::find()
                    .filter(permission::Column::Id.eq(permission_id.clone()))
                    .one(&self.db)
                    .await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to verify permission insert: {}", e)))?
                    .is_some()
                    || (code.is_some()
                        && permission::Entity::find()
                            .filter(permission::Column::Code.eq(code.clone().unwrap()))
                            .one(&self.db)
                            .await
                            .map_err(|e| {
                                AppError::DatabaseError(format!("Failed to verify permission code insert: {}", e))
                            })?
                            .is_some());
                if exists {
                    Ok(())
                } else {
                    Err(AppError::DatabaseError(format!("Failed to create permission: {}", e)))
                }
            }
        }
    }

    async fn save(&self, aggregate: &PermissionAggregate) -> AppResult<()> {
        let model = self.to_active_model(&aggregate.permission);
        permission::Entity::update(model)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save permission: {}", e)))?;
        Ok(())
    }

    async fn delete_by_id(&self, id: &PermissionId) -> AppResult<()> {
        use sea_orm::EntityTrait;
        permission::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete permission by id failed: {}", e)))?;
        Ok(())
    }
}
