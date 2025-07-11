use async_trait::async_trait;

use crate::entities::permission::Permission;
use crate::entities::role::Role;
use crate::value_objects::permission::PermissionId;
use crate::value_objects::role::{RoleId, RoleName};
use tradewinds_error::AppResult;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_id(&self, id: &RoleId) -> AppResult<Option<Role>>;
    async fn find_by_name(&self, name: &RoleName) -> AppResult<Option<Role>>;
    async fn find_by_ids(&self, ids: &[RoleId]) -> AppResult<Vec<Role>>;
    async fn exists_by_id(&self, id: &RoleId) -> AppResult<bool>;

    async fn find_with_permissions(&self, id: &RoleId) -> AppResult<Option<(Role, Vec<PermissionId>)>>;
    async fn find_permissions(&self, id: &RoleId) -> AppResult<Vec<Permission>>;
    async fn find_permissions_by_ids(&self, ids: &[RoleId]) -> AppResult<Vec<Permission>>;
    async fn search(
        &self,
        name: Option<&RoleName>,
        code: Option<&str>,
        status: Option<i32>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<Role>, u64)>;
}
