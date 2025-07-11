#[rustfmt::skip]
use crate::{
    commands::role::*,
    queries::*,
};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{permission::Permission, role::Role};
use tradewinds_error::AppResult;

/// 角色服务接口
///
/// 定义了角色服务的基本操作，包括创建、更新、删除、分配权限和撤销权限。
/// 这些操作通过命令和查询来实现。
///
/// 实现此接口的类型必须实现以下方法：
/// - `create_role`: 创建角色
/// - `update_role`: 更新角色
/// - `delete_role`: 删除角色
/// - `assign_permission`: 分配权限
/// - `revoke_permission`: 撤销权限
/// - `get_role_by_id`: 根据ID获取角色
/// - `get_role_by_name`: 根据名称获取角色
/// - `get_role_permissions`: 获取角色权限
/// - `list_roles`: 列出所有角色
#[async_trait::async_trait]
pub trait IRoleService: Send + Sync {
    async fn create_role(&self, cmd: CreateRoleCommand) -> AppResult<Role>;
    async fn update_role(&self, cmd: UpdateRoleCommand) -> AppResult<()>;
    async fn delete_role(&self, cmd: DeleteRoleCommand) -> AppResult<()>;
    async fn assign_permission(&self, cmd: AssignPermissionCommand) -> AppResult<()>;
    async fn revoke_permission(&self, cmd: RevokePermissionCommand) -> AppResult<()>;
    async fn get_role_by_id(&self, query: GetRoleByIdQuery) -> AppResult<Role>;
    async fn get_role_by_name(&self, query: GetRoleByNameQuery) -> AppResult<Role>;
    async fn get_role_permissions(&self, query: GetRolePermissionsQuery) -> AppResult<Vec<Permission>>;

    /// 分页获取角色列表（包含权限信息）
    /// 返回：(角色信息, 权限列表) 的元组集合
    async fn list_roles(&self, query: ListRolesQuery) -> AppResult<PaginatedResult<(Role, Vec<Permission>)>>;
}
