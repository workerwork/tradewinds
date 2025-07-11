use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait, Set,
};

use tradewinds_domain::entities::{permission::Permission, role::Role};
use tradewinds_domain::repositories::RoleRepository;
use tradewinds_domain::value_objects::permission::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType,
};
use tradewinds_domain::value_objects::role::{RoleCode, RoleId, RoleName};
use tradewinds_domain::value_objects::{RoleDescription, RoleStatus};

use crate::persistence::entities::{role, role_permission};
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct SeaOrmRoleRepository {
    db: DatabaseConnection,
}

impl SeaOrmRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn from_model(&self, model: role::Model) -> AppResult<Role> {
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

    fn to_active_model(&self, role: &Role) -> role::ActiveModel {
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
}

#[async_trait]
impl RoleRepository for SeaOrmRoleRepository {
    async fn find_by_id(&self, id: &RoleId) -> AppResult<Option<Role>> {
        role::Entity::find_by_id(id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find role by id failed: {}", e)))?
            .map(|model| self.from_model(model))
            .transpose()
    }

    async fn find_by_name(&self, name: &RoleName) -> AppResult<Option<Role>> {
        role::Entity::find()
            .filter(role::Column::Name.eq(name.value()))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find role by name failed: {}", e)))?
            .map(|model| self.from_model(model))
            .transpose()
    }

    async fn find_by_ids(&self, ids: &[RoleId]) -> AppResult<Vec<Role>> {
        let id_strs: Vec<String> = ids.iter().map(|id| id.value().to_string()).collect();
        role::Entity::find()
            .filter(role::Column::Id.is_in(id_strs))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find roles by ids failed: {}", e)))?
            .into_iter()
            .map(|model| self.from_model(model))
            .collect()
    }

    async fn exists_by_id(&self, id: &RoleId) -> AppResult<bool> {
        let count = role::Entity::find_by_id(id.value())
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Check if role exists by id failed: {}", e)))?;

        Ok(count > 0)
    }

    async fn find_with_permissions(&self, id: &RoleId) -> AppResult<Option<(Role, Vec<PermissionId>)>> {
        // TODO: 已彻底移除find_with_related相关调用，后续用join查询实现
        Ok(None)
    }

    async fn find_permissions(&self, id: &RoleId) -> AppResult<Vec<Permission>> {
        use crate::persistence::entities::permission;
        use crate::persistence::entities::role_permission;
        let perms = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(id.value()))
            .find_also_related(permission::Entity)
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permissions for role failed: {}", e)))?
            .into_iter()
            .filter_map(|(_, perm)| perm)
            .map(|p_model| {
                Ok(Permission {
                    id: PermissionId::new(p_model.id)?,
                    name: PermissionName::new(p_model.name)?,
                    code: p_model.code.map(PermissionCode::new).transpose()?,
                    type_: PermissionType::from_i32(p_model.type_)?,
                    parent_id: p_model.parent_id.map(PermissionId::new).transpose()?,
                    path: p_model.path.map(PermissionPath::new).transpose()?,
                    component: p_model.component.map(PermissionComponent::new).transpose()?,
                    icon: p_model.icon.map(PermissionIcon::new).transpose()?,
                    sort: PermissionSort::new(p_model.sort)?,
                    status: PermissionStatus::from_i32(p_model.status)?,
                    created_at: p_model.created_at.timestamp(),
                    updated_at: p_model.updated_at.timestamp(),
                })
            })
            .collect::<AppResult<Vec<_>>>()?;
        Ok(perms)
    }

    async fn find_permissions_by_ids(&self, ids: &[RoleId]) -> AppResult<Vec<Permission>> {
        // This is a more complex query. A common way is to fetch all permissions for the given role IDs
        // and then group them. However, for simplicity, we can iterate and fetch one by one.
        // For production, a more optimized query (e.g., using a single query with a subquery or join) would be better.
        let mut all_perms = Vec::new();
        for id in ids {
            let perms = self.find_permissions(id).await?;
            all_perms.extend(perms);
        }
        // This will contain duplicates if multiple roles have the same permission.
        // Depending on requirements, you might want to deduplicate.
        Ok(all_perms)
    }

    async fn search(
        &self,
        name: Option<&RoleName>,
        code: Option<&str>,
        status: Option<i32>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<Role>, u64)> {
        let mut query = role::Entity::find();
        if let Some(name) = name {
            query = query.filter(role::Column::Name.contains(name.value()));
        }
        if let Some(code) = code {
            query = query.filter(role::Column::Code.contains(code));
        }
        if let Some(status) = status {
            query = query.filter(role::Column::Status.eq(status));
        } else if show_deleted == Some(true) {
            // 不加 status 过滤，查全部
        } else {
            // 只查未删除（0/1）
            query = query.filter(role::Column::Status.is_in([0, 1]));
        }
        let total = query
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Count roles failed: {}", e)))?;
        let models = query
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("List roles failed: {}", e)))?;
        let roles = models.into_iter().map(|model| self.from_model(model)).collect::<AppResult<Vec<Role>>>()?;
        Ok((roles, total))
    }
}
