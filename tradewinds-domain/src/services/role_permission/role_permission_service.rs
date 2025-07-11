use crate::value_objects::{PermissionId, RoleId};
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait RolePermissionService: Send + Sync {
    /// 给角色分配权限
    async fn assign_permission(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<()>;

    /// 从角色撤销权限
    async fn revoke_permission(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<()>;

    /// 查询角色拥有的所有权限ID
    async fn list_permissions_by_role(&self, role_id: &RoleId) -> AppResult<Vec<PermissionId>>;

    /// 判断角色是否拥有指定权限
    async fn has_permission(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<bool>;
}
