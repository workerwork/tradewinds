use std::sync::Arc;

use tradewinds_application::commands::role::handlers::{
    AssignPermissionHandler, CreateRoleHandler, DeleteRoleHandler, RevokePermissionHandler, UpdateRoleHandler,
};
use tradewinds_application::commands::role::*;
use tradewinds_application::interfaces::IRoleService;
use tradewinds_application::queries::role::handlers::{
    GetRoleByIdHandler, GetRoleByNameHandler, GetRolePermissionsHandler, ListRolesHandler,
};
use tradewinds_application::queries::role::*;
use tradewinds_application::{CommandHandler, QueryHandler};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{permission::Permission, role::Role};
use tradewinds_error::AppResult;

#[rustfmt::skip]
use crate::api::{
    dtos::permission_dto::PermissionResponse, 
    dtos::role_dto::*, 
    mappers::role_mapper,
};

/// 角色控制器
/// 负责协调角色相关的用例
pub struct RoleController {
    create_role: Arc<dyn CommandHandler<CreateRoleCommand, Role>>,
    update_role: Arc<dyn CommandHandler<UpdateRoleCommand, ()>>,
    delete_role: Arc<dyn CommandHandler<DeleteRoleCommand, ()>>,
    assign_permission: Arc<dyn CommandHandler<AssignPermissionCommand, ()>>,
    revoke_permission: Arc<dyn CommandHandler<RevokePermissionCommand, ()>>,
    get_role_by_id: Arc<dyn QueryHandler<GetRoleByIdQuery, Role>>,
    get_role_by_name: Arc<dyn QueryHandler<GetRoleByNameQuery, Role>>,
    get_role_permissions: Arc<dyn QueryHandler<GetRolePermissionsQuery, Vec<Permission>>>,
    list_roles: Arc<dyn QueryHandler<ListRolesQuery, PaginatedResult<(Role, Vec<Permission>)>>>,
}

impl RoleController {
    pub fn new(
        create_role: Arc<dyn CommandHandler<CreateRoleCommand, Role>>,
        update_role: Arc<dyn CommandHandler<UpdateRoleCommand, ()>>,
        delete_role: Arc<dyn CommandHandler<DeleteRoleCommand, ()>>,
        assign_permission: Arc<dyn CommandHandler<AssignPermissionCommand, ()>>,
        revoke_permission: Arc<dyn CommandHandler<RevokePermissionCommand, ()>>,
        get_role_by_id: Arc<dyn QueryHandler<GetRoleByIdQuery, Role>>,
        get_role_by_name: Arc<dyn QueryHandler<GetRoleByNameQuery, Role>>,
        get_role_permissions: Arc<dyn QueryHandler<GetRolePermissionsQuery, Vec<Permission>>>,
        list_roles: Arc<dyn QueryHandler<ListRolesQuery, PaginatedResult<(Role, Vec<Permission>)>>>,
    ) -> Self {
        Self {
            create_role,
            update_role,
            delete_role,
            assign_permission,
            revoke_permission,
            get_role_by_id,
            get_role_by_name,
            get_role_permissions,
            list_roles,
        }
    }

    pub fn assemble(role_service: Arc<dyn IRoleService>) -> Self {
        Self::new(
            Arc::new(CreateRoleHandler::new(role_service.clone())),
            Arc::new(UpdateRoleHandler::new(role_service.clone())),
            Arc::new(DeleteRoleHandler::new(role_service.clone())),
            Arc::new(AssignPermissionHandler::new(role_service.clone())),
            Arc::new(RevokePermissionHandler::new(role_service.clone())),
            Arc::new(GetRoleByIdHandler::new(role_service.clone())),
            Arc::new(GetRoleByNameHandler::new(role_service.clone())),
            Arc::new(GetRolePermissionsHandler::new(role_service.clone())),
            Arc::new(ListRolesHandler::new(role_service.clone())),
        )
    }

    /// 创建角色
    pub async fn create_role(&self, actor_id: String, req: CreateRoleRequest) -> AppResult<CreateRoleResponse> {
        let command = role_mapper::to_create_role_command(req)?;
        let role = self.create_role.handle(command).await?;
        Ok(CreateRoleResponse { role: role.into() })
    }

    /// 更新角色
    pub async fn update_role(&self, actor_id: String, req: UpdateRoleRequest) -> AppResult<UpdateRoleResponse> {
        let command = role_mapper::to_update_role_command(actor_id, req)?;
        self.update_role.handle(command).await?;
        Ok(UpdateRoleResponse)
    }

    /// 删除角色
    pub async fn delete_role(&self, actor_id: String, req: DeleteRoleRequest) -> AppResult<DeleteRoleResponse> {
        let command = role_mapper::to_delete_role_command(actor_id, req)?;
        self.delete_role.handle(command).await?;
        Ok(DeleteRoleResponse)
    }

    /// 分配权限
    pub async fn assign_permission(
        &self,
        actor_id: String,
        req: AssignPermissionRequest,
    ) -> AppResult<AssignPermissionResponse> {
        let command = role_mapper::to_assign_permission_command(actor_id, req)?;
        self.assign_permission.handle(command).await?;
        Ok(AssignPermissionResponse)
    }

    /// 撤销权限
    pub async fn revoke_permission(
        &self,
        actor_id: String,
        req: RevokePermissionRequest,
    ) -> AppResult<RevokePermissionResponse> {
        let command = role_mapper::to_revoke_permission_command(actor_id, req)?;
        self.revoke_permission.handle(command).await?;
        Ok(RevokePermissionResponse)
    }

    /// 通过ID获取角色
    pub async fn get_role_by_id(&self, req: GetRoleByIdRequest) -> AppResult<GetRoleByIdResponse> {
        let query = role_mapper::to_get_role_by_id_query(req)?;
        let role = self.get_role_by_id.handle(query).await?;
        Ok(GetRoleByIdResponse { role: role.into() })
    }

    /// 通过名称获取角色
    pub async fn get_role_by_name(&self, req: GetRoleByNameRequest) -> AppResult<GetRoleByNameResponse> {
        let query = role_mapper::to_get_role_by_name_query(req)?;
        let role = self.get_role_by_name.handle(query).await?;
        Ok(GetRoleByNameResponse { role: role.into() })
    }

    /// 获取角色的权限列表
    pub async fn get_role_permissions(&self, req: GetRolePermissionsRequest) -> AppResult<GetRolePermissionsResponse> {
        let query = role_mapper::to_get_role_permissions_query(req)?;
        let permissions = self.get_role_permissions.handle(query).await?;
        Ok(GetRolePermissionsResponse {
            permissions: permissions.into_iter().map(PermissionResponse::from).collect(), // FIXME
        })
    }

    /// 获取角色列表
    pub async fn list_roles(&self, req: ListRolesRequest) -> AppResult<ListRolesResponse> {
        let query = role_mapper::to_list_roles_query(req)?;
        let result = self.list_roles.handle(query).await?;
        let roles = result
            .items
            .into_iter()
            .map(|(role, permissions)| RoleWithPermissionsResponse::new(role, permissions))
            .collect();
        Ok(ListRolesResponse { roles, total: result.total })
    }
}
