use async_trait::async_trait;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{permission::Permission, role::Role, user::User};
use tradewinds_error::AppResult;

use crate::commands::user::*;
use crate::queries::user::*;

/// 用户服务接口
///
/// 该接口定义了用户相关的所有业务操作，包括：
/// - 用户CRUD操作
/// - 角色分配和撤销
/// - 用户查询（通过ID、用户名、邮箱）
/// - 用户角色和权限查询
/// - 分页查询功能
///
/// 所有操作都是异步的，返回 AppResult<T> 来处理错误情况。
///
/// 主要方法：
/// - `create_user`: 创建新用户
/// - `update_user`: 更新用户信息
/// - `delete_user`: 删除用户
/// - `assign_role`: 为用户分配角色
/// - `revoke_role`: 撤销用户角色
/// - `get_user_by_id/username/email`: 根据不同条件查询用户
/// - `get_user_roles`: 获取用户的角色列表
/// - `get_user_permissions`: 获取用户的权限列表
/// - `list_users`: 列出所有用户
#[async_trait]
pub trait IUserService: Send + Sync {
    /// 创建用户
    async fn create_user(&self, cmd: CreateUserCommand) -> AppResult<User>;

    /// 更新用户
    async fn update_user(&self, cmd: UpdateUserCommand) -> AppResult<()>;

    /// 删除用户
    async fn delete_user(&self, cmd: DeleteUserCommand) -> AppResult<()>;

    /// 为用户分配角色
    async fn assign_role(&self, cmd: AssignRoleCommand) -> AppResult<()>;

    /// 撤销用户角色
    async fn revoke_role(&self, cmd: RevokeRoleCommand) -> AppResult<()>;

    /// 重置用户密码
    async fn reset_password(&self, cmd: ResetPasswordCommand) -> AppResult<()>;

    /// 根据ID获取用户
    async fn get_user_by_id(&self, query: GetUserByIdQuery) -> AppResult<User>;

    /// 根据用户名获取用户
    async fn get_user_by_username(&self, query: GetUserByUsernameQuery) -> AppResult<User>;

    /// 根据邮箱获取用户
    async fn get_user_by_email(&self, query: GetUserByEmailQuery) -> AppResult<User>;

    /// 获取用户的角色列表
    async fn get_user_roles(&self, query: GetUserRolesQuery) -> AppResult<Vec<Role>>;

    /// 获取用户的权限列表
    async fn get_user_permissions(&self, query: GetUserPermissionsQuery) -> AppResult<Vec<Permission>>;

    /// 分页获取用户列表（包含角色信息）
    /// 返回：(用户信息, 角色列表) 的元组集合
    async fn list_users(&self, query: ListUsersQuery) -> AppResult<PaginatedResult<(User, Vec<Role>)>>;
}
