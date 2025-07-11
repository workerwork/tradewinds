use crate::entities::Role;
use crate::value_objects::role::RoleId;
use tradewinds_error::AppResult;

use async_trait::async_trait;

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn create(&self, role: &Role) -> AppResult<()>;
    async fn update(&self, role: &Role) -> AppResult<()>;
    async fn delete(&self, id: &RoleId) -> AppResult<()>;
    async fn get_by_id(&self, id: &RoleId) -> AppResult<Role>;
    async fn get_by_ids(&self, ids: &[RoleId]) -> AppResult<Vec<Role>>;
    async fn get_by_name(&self, name: &str) -> AppResult<Role>;
    async fn list(&self, page: u32, page_size: u32) -> AppResult<Vec<Role>>;
    async fn exists_by_name(&self, name: &str) -> AppResult<bool>;
}
