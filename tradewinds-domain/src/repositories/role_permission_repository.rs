use async_trait::async_trait;
use tradewinds_error::AppResult;

use crate::entities::role_permission::RolePermission;
use crate::value_objects::{PermissionId, RoleId};

#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn create(&self, role_permissions: &[RolePermission]) -> AppResult<()>;
    async fn delete(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<()>;
    async fn find_by_role_id(&self, role_id: &RoleId) -> AppResult<Vec<RolePermission>>;
    async fn find_by_permission_id(&self, permission_id: &PermissionId) -> AppResult<Vec<RolePermission>>;
    async fn exists(&self, role_id: &RoleId, permission_id: &PermissionId) -> AppResult<bool>;
}
