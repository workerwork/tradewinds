use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    SelectorTrait, Set,
};

use crate::persistence::entities::role_permission;
use tradewinds_domain::entities::RolePermission;
use tradewinds_domain::repositories::RolePermissionRepository;
use tradewinds_domain::value_objects::{PermissionId, RoleId, RolePermissionId};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct SeaOrmRolePermissionRepository {
    db: DatabaseConnection,
}

impl SeaOrmRolePermissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn from_model(&self, model: role_permission::Model) -> AppResult<RolePermission> {
        Ok(RolePermission {
            id: RolePermissionId::new(model.id)?,
            role_id: RoleId::new(model.role_id)?,
            permission_id: PermissionId::new(model.permission_id)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn to_active_model(&self, role_permission: &RolePermission) -> role_permission::ActiveModel {
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
impl RolePermissionRepository for SeaOrmRolePermissionRepository {
    async fn create(&self, role_permissions: &[RolePermission]) -> AppResult<()> {
        let models: Vec<role_permission::ActiveModel> =
            role_permissions.iter().map(|rp| self.to_active_model(rp)).collect();

        role_permission::Entity::insert_many(models)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Assign permissions failed: {}", e)))?;
        Ok(())
    }

    async fn delete(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<()> {
        role_permission::Entity::delete_many()
            .filter(role_permission::Column::RoleId.eq(role_id.value()))
            .filter(role_permission::Column::PermissionId.eq(permission_id.value()))
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Revoke permissions failed: {}", e)))?;
        Ok(())
    }

    async fn find_by_role_id(&self, role_id: &RoleId) -> AppResult<Vec<RolePermission>> {
        role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id.value()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permissions by role id failed: {}", e)))?
            .into_iter()
            .map(|m| self.from_model(m))
            .collect()
    }

    async fn find_by_permission_id(&self, permission_id: &PermissionId) -> AppResult<Vec<RolePermission>> {
        role_permission::Entity::find()
            .filter(role_permission::Column::PermissionId.eq(permission_id.value()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permissions by permission id failed: {}", e)))?
            .into_iter()
            .map(|m| self.from_model(m))
            .collect()
    }

    async fn exists(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<bool> {
        let count = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id.value()))
            .filter(role_permission::Column::PermissionId.eq(permission_id.value()))
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Check permission existence failed: {}", e)))?;
        Ok(count > 0)
    }
}
