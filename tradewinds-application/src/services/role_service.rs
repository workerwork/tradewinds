use crate::commands::role::{
    AssignPermissionCommand, CreateRoleCommand, DeleteRoleCommand, RevokePermissionCommand, UpdateRoleCommand,
};
use crate::interfaces::IRoleService;
use crate::queries::role::{GetRoleByIdQuery, GetRoleByNameQuery, GetRolePermissionsQuery, ListRolesQuery};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::{
    aggregates::role_aggregate::RoleAggregate,
    entities::{permission::Permission, role::Role},
    repositories::{RoleAggregateRepository, RolePermissionRepository, RoleRepository},
    value_objects::role::RoleCode,
    value_objects::role::RoleName,
    value_objects::role::RoleStatus,
};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct RoleService {
    role_repo: Arc<dyn RoleRepository>,
    role_agg_repo: Arc<dyn RoleAggregateRepository>,
}

impl RoleService {
    pub fn new(role_repo: Arc<dyn RoleRepository>, role_agg_repo: Arc<dyn RoleAggregateRepository>) -> Self {
        Self { role_repo, role_agg_repo }
    }
}

#[async_trait::async_trait]
impl IRoleService for RoleService {
    async fn list_roles(&self, query: ListRolesQuery) -> AppResult<PaginatedResult<(Role, Vec<Permission>)>> {
        let (page_size, offset) = query.pagination();
        let name_ref = query.name.as_ref().and_then(|s| RoleName::new(s.clone()).ok());
        let (roles, total) = self
            .role_repo
            .search(name_ref.as_ref(), query.code.as_deref(), query.status, query.show_deleted, page_size, offset)
            .await?;

        // 为每个角色查询权限信息
        let mut roles_with_permissions = Vec::new();
        for role in roles {
            let permissions = self.role_repo.find_permissions(&role.id).await?;
            roles_with_permissions.push((role, permissions));
        }

        Ok(PaginatedResult { items: roles_with_permissions, total })
    }

    async fn create_role(&self, cmd: CreateRoleCommand) -> AppResult<Role> {
        if self.role_repo.find_by_name(&cmd.name).await?.is_some() {
            return Err(AppError::Validation("Role name already exists".into()));
        }

        let code = cmd.code.clone();
        let status = cmd.status.unwrap_or_default();
        let role_agg = RoleAggregate::create(cmd.name, code, cmd.description, Some(cmd.permissions), status)?;

        self.role_agg_repo.create(&role_agg).await?;

        Ok(role_agg.role)
    }

    async fn update_role(&self, cmd: UpdateRoleCommand) -> AppResult<()> {
        let mut role_agg = self
            .role_agg_repo
            .find_by_id(&cmd.id)
            .await?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

        role_agg.update(cmd.name, cmd.description, cmd.status, cmd.permissions)?;

        self.role_agg_repo.save(&role_agg).await?;

        Ok(())
    }

    async fn delete_role(&self, cmd: DeleteRoleCommand) -> AppResult<()> {
        self.role_agg_repo.delete_by_id(&cmd.id).await?;
        Ok(())
    }

    async fn assign_permission(&self, cmd: AssignPermissionCommand) -> AppResult<()> {
        let mut role_agg = self
            .role_agg_repo
            .find_by_id(&cmd.role_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

        for permission_id in cmd.permission_ids {
            role_agg.assign_permission(&permission_id)?;
        }

        self.role_agg_repo.save(&role_agg).await?;

        Ok(())
    }

    async fn revoke_permission(&self, cmd: RevokePermissionCommand) -> AppResult<()> {
        let mut role_agg = self
            .role_agg_repo
            .find_by_id(&cmd.role_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

        role_agg.revoke_permission(&cmd.permission_id);

        self.role_agg_repo.save(&role_agg).await?;

        Ok(())
    }

    async fn get_role_by_id(&self, query: GetRoleByIdQuery) -> AppResult<Role> {
        self.role_repo.find_by_id(&query.role_id).await?.ok_or_else(|| AppError::NotFound("Role not found".to_string()))
    }

    async fn get_role_by_name(&self, query: GetRoleByNameQuery) -> AppResult<Role> {
        self.role_repo
            .find_by_name(&query.role_name)
            .await?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))
    }

    async fn get_role_permissions(&self, query: GetRolePermissionsQuery) -> AppResult<Vec<Permission>> {
        self.role_repo.find_permissions(&query.role_id).await
    }
}
