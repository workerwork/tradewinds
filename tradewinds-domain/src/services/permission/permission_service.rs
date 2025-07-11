use crate::entities::permission::Permission;
use crate::value_objects::permission::PermissionId;
use tradewinds_error::AppResult;

use async_trait::async_trait;

#[async_trait]
pub trait PermissionService: Send + Sync {
    async fn create(&self, permission: &Permission) -> AppResult<()>;
    async fn update(&self, permission: &Permission) -> AppResult<()>;
    async fn delete(&self, id: &PermissionId) -> AppResult<()>;
    async fn get_by_id(&self, id: &PermissionId) -> AppResult<Permission>;
    async fn get_by_ids(&self, ids: &[PermissionId]) -> AppResult<Vec<Permission>>;
    async fn get_by_name(&self, name: &str) -> AppResult<Permission>;
    async fn list(&self, page: u32, page_size: u32) -> AppResult<Vec<Permission>>;
    async fn exists_by_name(&self, name: &str) -> AppResult<bool>;
}
