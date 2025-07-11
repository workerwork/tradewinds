use crate::aggregates::permission_aggregate::PermissionAggregate;
use crate::value_objects::permission::PermissionId;
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait PermissionAggregateRepository: Send + Sync {
    async fn create(&self, aggregate: &PermissionAggregate) -> AppResult<()>;
    async fn save(&self, aggregate: &PermissionAggregate) -> AppResult<()>;
    async fn find_by_id(&self, id: &PermissionId) -> AppResult<Option<PermissionAggregate>>;
    async fn delete_by_id(&self, id: &PermissionId) -> AppResult<()>;
}
