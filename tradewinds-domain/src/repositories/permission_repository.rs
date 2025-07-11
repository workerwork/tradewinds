use async_trait::async_trait;
use tradewinds_error::AppResult;

use crate::entities::permission::Permission;
use crate::value_objects::permission::{
    PermissionCode, PermissionId, PermissionName, PermissionStatus, PermissionType,
};
use crate::value_objects::user::UserId;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_by_id(&self, id: &PermissionId) -> AppResult<Option<Permission>>;
    async fn find_by_name(&self, name: &PermissionName) -> AppResult<Option<Permission>>;
    async fn find_by_code(&self, code: &PermissionCode) -> AppResult<Option<Permission>>;
    async fn find_by_ids(&self, ids: &[PermissionId]) -> AppResult<Vec<Permission>>;
    async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<Permission>>;
    async fn search(
        &self,
        name: Option<&PermissionName>,
        code: Option<&PermissionCode>,
        permission_type: Option<&PermissionType>,
        status: Option<PermissionStatus>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<Permission>, u64)>;
    async fn find_all(&self) -> AppResult<Vec<Permission>>;
}
