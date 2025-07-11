use std::sync::Arc;

use tradewinds_application::commands::user::handlers::{
    AssignRoleHandler, CreateUserHandler, DeleteUserHandler, ResetPasswordHandler, RevokeRoleHandler, UpdateUserHandler,
};
use tradewinds_application::commands::user::*;
use tradewinds_application::interfaces::{ISystemSettingService, IUserService};
use tradewinds_application::queries::user::handlers::{
    GetUserByEmailHandler, GetUserByIdHandler, GetUserByUsernameHandler, GetUserPermissionsHandler,
    GetUserRolesHandler, ListUsersHandler,
};
use tradewinds_application::queries::user::*;
use tradewinds_application::{CommandHandler, QueryHandler};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{permission::Permission, role::Role, user::User};
use tradewinds_error::AppResult;

#[rustfmt::skip]
use crate::api::{
    dtos::user_dto::*,
    mappers::user_mapper,
};

/// 用户控制器，负责协调用户相关的用例
pub struct UserController {
    create_user: Arc<dyn CommandHandler<CreateUserCommand, User>>,
    update_user: Arc<dyn CommandHandler<UpdateUserCommand, ()>>,
    delete_user: Arc<dyn CommandHandler<DeleteUserCommand, ()>>,
    assign_role: Arc<dyn CommandHandler<AssignRoleCommand, ()>>,
    revoke_role: Arc<dyn CommandHandler<RevokeRoleCommand, ()>>,
    reset_password: Arc<dyn CommandHandler<ResetPasswordCommand, ()>>,
    get_user_by_id: Arc<dyn QueryHandler<GetUserByIdQuery, User>>,
    get_user_by_username: Arc<dyn QueryHandler<GetUserByUsernameQuery, User>>,
    get_user_by_email: Arc<dyn QueryHandler<GetUserByEmailQuery, User>>,
    get_user_roles: Arc<dyn QueryHandler<GetUserRolesQuery, Vec<Role>>>,
    get_user_permissions: Arc<dyn QueryHandler<GetUserPermissionsQuery, Vec<Permission>>>,
    list_users: Arc<dyn QueryHandler<ListUsersQuery, PaginatedResult<(User, Vec<Role>)>>>,
}

impl UserController {
    pub fn new(
        create_user: Arc<dyn CommandHandler<CreateUserCommand, User>>,
        update_user: Arc<dyn CommandHandler<UpdateUserCommand, ()>>,
        delete_user: Arc<dyn CommandHandler<DeleteUserCommand, ()>>,
        assign_role: Arc<dyn CommandHandler<AssignRoleCommand, ()>>,
        revoke_role: Arc<dyn CommandHandler<RevokeRoleCommand, ()>>,
        reset_password: Arc<dyn CommandHandler<ResetPasswordCommand, ()>>,
        get_user_by_id: Arc<dyn QueryHandler<GetUserByIdQuery, User>>,
        get_user_by_username: Arc<dyn QueryHandler<GetUserByUsernameQuery, User>>,
        get_user_by_email: Arc<dyn QueryHandler<GetUserByEmailQuery, User>>,
        get_user_roles: Arc<dyn QueryHandler<GetUserRolesQuery, Vec<Role>>>,
        get_user_permissions: Arc<dyn QueryHandler<GetUserPermissionsQuery, Vec<Permission>>>,
        list_users: Arc<dyn QueryHandler<ListUsersQuery, PaginatedResult<(User, Vec<Role>)>>>,
    ) -> Self {
        Self {
            create_user,
            update_user,
            delete_user,
            assign_role,
            revoke_role,
            reset_password,
            get_user_by_id,
            get_user_by_username,
            get_user_by_email,
            get_user_roles,
            get_user_permissions,
            list_users,
        }
    }

    /// 创建用户
    pub async fn create_user(&self, actor_id: String, req: CreateUserRequest) -> AppResult<CreateUserResponse> {
        let command = user_mapper::to_create_user_command(actor_id, req)?;
        let user = self.create_user.handle(command).await?;
        Ok(CreateUserResponse { user: user.into() })
    }

    /// 更新用户
    pub async fn update_user(&self, actor_id: String, req: UpdateUserRequest) -> AppResult<UpdateUserResponse> {
        let command = user_mapper::to_update_user_command(actor_id, req)?;
        self.update_user.handle(command).await?;
        Ok(UpdateUserResponse)
    }

    /// 删除用户
    pub async fn delete_user(&self, actor_id: String, req: DeleteUserRequest) -> AppResult<DeleteUserResponse> {
        let command = user_mapper::to_delete_user_command(actor_id, req)?;
        self.delete_user.handle(command).await?;
        Ok(DeleteUserResponse)
    }

    /// 重置密码
    pub async fn reset_password(&self, actor_id: String, id: String) -> AppResult<ResetPasswordResponse> {
        let command = user_mapper::to_reset_password_command(actor_id, id)?;
        self.reset_password.handle(command).await?;
        Ok(ResetPasswordResponse)
    }

    /// 分配角色
    pub async fn assign_role(&self, actor_id: String, req: AssignRoleRequest) -> AppResult<AssignRoleResponse> {
        let command = user_mapper::to_assign_role_command(actor_id, req)?;
        self.assign_role.handle(command).await?;
        Ok(AssignRoleResponse)
    }

    /// 撤销角色
    pub async fn revoke_role(&self, actor_id: String, req: RevokeRoleRequest) -> AppResult<RevokeRoleResponse> {
        let command = user_mapper::to_revoke_role_command(actor_id, req)?;
        let _ = self.revoke_role.handle(command).await?;
        Ok(RevokeRoleResponse)
    }

    /// 根据ID获取用户
    pub async fn get_user_by_id(&self, req: GetUserByIdRequest) -> AppResult<GetUserByIdResponse> {
        let query = user_mapper::to_get_user_by_id_query(req)?;
        let user = self.get_user_by_id.handle(query).await?;
        Ok(GetUserByIdResponse { user: user.into() })
    }

    /// 根据用户名获取用户
    pub async fn get_user_by_username(&self, req: GetUserByUsernameRequest) -> AppResult<GetUserByUsernameResponse> {
        let query = user_mapper::to_get_user_by_username_query(req)?;
        let user = self.get_user_by_username.handle(query).await?;
        Ok(GetUserByUsernameResponse { user: user.into() })
    }

    /// 根据邮箱获取用户
    pub async fn get_user_by_email(&self, req: GetUserByEmailRequest) -> AppResult<GetUserByEmailResponse> {
        // FIXME: 需要验证邮箱是否存在
        let query = user_mapper::to_get_user_by_email_query(req)?;
        let user = self.get_user_by_email.handle(query).await?;
        Ok(GetUserByEmailResponse { user: user.into() })
    }

    /// 获取用户角色列表
    pub async fn get_user_roles(&self, req: GetUserRolesRequest) -> AppResult<GetUserRolesResponse> {
        let query = user_mapper::to_get_user_roles_query(req)?;
        let roles = self.get_user_roles.handle(query).await?;
        Ok(GetUserRolesResponse { roles: roles.into_iter().map(Into::into).collect() })
    }

    /// 获取用户权限列表
    pub async fn get_user_permissions(&self, req: GetUserPermissionsRequest) -> AppResult<GetUserPermissionsResponse> {
        let query = user_mapper::to_get_user_permissions_query(req)?;
        let permissions = self.get_user_permissions.handle(query).await?;
        Ok(GetUserPermissionsResponse { permissions: permissions.into_iter().map(Into::into).collect() })
    }

    /// 获取用户列表
    pub async fn list_users(&self, req: ListUsersRequest) -> AppResult<ListUsersResponse> {
        let query = user_mapper::to_list_users_query(req)?;
        let result = self.list_users.handle(query).await?;
        let users = result.items.into_iter().map(|(user, roles)| UserWithRolesResponse::new(user, roles)).collect();
        Ok(ListUsersResponse { users, total: result.total })
    }

    pub fn assemble(
        user_service: Arc<dyn IUserService>,
        system_setting_service: Arc<dyn ISystemSettingService>,
    ) -> Self {
        Self::new(
            Arc::new(CreateUserHandler::new(user_service.clone())),
            Arc::new(UpdateUserHandler::new(user_service.clone())),
            Arc::new(DeleteUserHandler::new(user_service.clone())),
            Arc::new(AssignRoleHandler::new(user_service.clone())),
            Arc::new(RevokeRoleHandler::new(user_service.clone())),
            Arc::new(ResetPasswordHandler::new(user_service.clone(), system_setting_service.clone())),
            Arc::new(GetUserByIdHandler::new(user_service.clone())),
            Arc::new(GetUserByUsernameHandler::new(user_service.clone())),
            Arc::new(GetUserByEmailHandler::new(user_service.clone())),
            Arc::new(GetUserRolesHandler::new(user_service.clone())),
            Arc::new(GetUserPermissionsHandler::new(user_service.clone())),
            Arc::new(ListUsersHandler::new(user_service.clone())),
        )
    }
}
