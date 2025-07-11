#[rustfmt::skip]
use crate::{
    commands::permission::*,
    queries::*,
};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 权限服务接口
///
/// 定义了权限服务的基本操作，包括创建、更新、删除、获取和列出权限。
/// 这些操作通过命令和查询来实现。
///
/// 实现此接口的类型必须实现以下方法：
/// - `create_permission`: 创建权限
/// - `update_permission`: 更新权限
/// - `delete_permission`: 删除权限
/// - `get_permission_by_id`: 根据ID获取权限
/// - `get_permission_by_name`: 根据名称获取权限
/// - `get_permission_by_code`: 根据代码获取权限
/// - `list_permissions`: 列出所有权限
/// - `list_permissions_by_type`: 根据类型列出权限
/// - `list_permissions_by_parent_id`: 根据父ID列出权限
#[async_trait::async_trait]
pub trait IPermissionService: Send + Sync {
    async fn create_permission(&self, cmd: CreatePermissionCommand) -> AppResult<Permission>;
    async fn update_permission(&self, cmd: UpdatePermissionCommand) -> AppResult<()>;
    async fn delete_permission(&self, cmd: DeletePermissionCommand) -> AppResult<()>;
    async fn get_permission_by_id(&self, query: GetPermissionByIdQuery) -> AppResult<Permission>;
    async fn get_permission_by_name(&self, query: GetPermissionByNameQuery) -> AppResult<Permission>;
    async fn get_permission_by_code(&self, query: GetPermissionByCodeQuery) -> AppResult<Permission>;
    async fn list_permissions(&self, query: ListPermissionsQuery) -> AppResult<PaginatedResult<Permission>>;
    async fn list_permissions_by_type(
        &self,
        query: ListPermissionsByTypeQuery,
    ) -> AppResult<PaginatedResult<Permission>>;
    async fn list_permissions_by_parent_id(
        &self,
        query: ListPermissionsByParentIdQuery,
    ) -> AppResult<PaginatedResult<Permission>>;
    /// 查询所有权限（不分页）
    async fn list_all_permissions(&self) -> AppResult<Vec<Permission>>;
}
