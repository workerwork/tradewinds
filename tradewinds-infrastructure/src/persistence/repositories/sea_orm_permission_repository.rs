use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait, Set,
};

use tradewinds_domain::entities::permission::Permission;
use tradewinds_domain::repositories::PermissionRepository;
use tradewinds_domain::value_objects::UserId;
use tradewinds_domain::value_objects::permission::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType,
};

use crate::persistence::entities::{permission, role, role_permission, user_role};
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct SeaOrmPermissionRepository {
    db: DatabaseConnection,
}

impl SeaOrmPermissionRepository {
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
impl PermissionRepository for SeaOrmPermissionRepository {
    async fn find_by_id(&self, id: &PermissionId) -> AppResult<Option<Permission>> {
        permission::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permission by id failed: {}", e)))?
            .map(|m| self.from_model(m))
            .transpose()
    }

    async fn find_by_name(&self, name: &PermissionName) -> AppResult<Option<Permission>> {
        permission::Entity::find()
            .filter(permission::Column::Name.eq(name.value()))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permission by name failed: {}", e)))?
            .map(|m| self.from_model(m))
            .transpose()
    }

    async fn find_by_code(&self, code: &PermissionCode) -> AppResult<Option<Permission>> {
        permission::Entity::find()
            .filter(permission::Column::Code.eq(code.value()))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permission by code failed: {}", e)))?
            .map(|m| self.from_model(m))
            .transpose()
    }

    async fn find_by_ids(&self, ids: &[PermissionId]) -> AppResult<Vec<Permission>> {
        let id_strs: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        permission::Entity::find()
            .filter(permission::Column::Id.is_in(id_strs))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permissions by ids failed: {}", e)))?
            .into_iter()
            .map(|m| self.from_model(m))
            .collect()
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<Permission>> {
        use crate::persistence::entities::{role_permission, user_role};

        // 1. 查找用户的角色ID
        let role_ids: Vec<String> = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id.to_string()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find user roles failed: {}", e)))?
            .into_iter()
            .map(|ur| ur.role_id)
            .collect();

        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        // 2. 查找角色的权限ID
        let permission_ids: Vec<String> = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(role_ids))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find role permissions failed: {}", e)))?
            .into_iter()
            .map(|rp| rp.permission_id)
            .collect();

        if permission_ids.is_empty() {
            return Ok(vec![]);
        }

        // 3. 查找权限详情
        let permissions = permission::Entity::find()
            .filter(permission::Column::Id.is_in(permission_ids))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find permissions failed: {}", e)))?;

        permissions.into_iter().map(|m| self.from_model(m)).collect::<Result<Vec<_>, _>>()
    }

    async fn find_all(&self) -> AppResult<Vec<Permission>> {
        let models = permission::Entity::find().all(&self.db).await?;
        models.into_iter().map(|m| self.from_model(m)).collect()
    }

    async fn search(
        &self,
        name: Option<&PermissionName>,
        code: Option<&PermissionCode>,
        permission_type: Option<&PermissionType>,
        status: Option<PermissionStatus>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<Permission>, u64)> {
        let mut query = permission::Entity::find();
        if let Some(name) = name {
            query = query.filter(permission::Column::Name.contains(name.value()));
        }
        if let Some(code) = code {
            query = query.filter(permission::Column::Code.contains(code.value()));
        }
        if let Some(permission_type) = permission_type {
            query = query.filter(permission::Column::Type.eq(permission_type.value()));
        }
        if let Some(status) = status {
            query = query.filter(permission::Column::Status.eq(status.value()));
        } else if show_deleted == Some(true) {
            // 不加 status 过滤，查全部
        } else {
            // 只查未删除（0/1）
            query = query.filter(permission::Column::Status.is_in([0, 1]));
        }
        let total = query
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Count permissions failed: {}", e)))?;
        let models = query
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("List permissions failed: {}", e)))?;
        let permissions = models.into_iter().map(|m| self.from_model(m)).collect::<AppResult<Vec<_>>>()?;
        Ok((permissions, total))
    }
}
