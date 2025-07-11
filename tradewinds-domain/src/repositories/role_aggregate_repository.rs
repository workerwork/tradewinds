use crate::aggregates::role_aggregate::RoleAggregate;
use crate::value_objects::role::RoleId;
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait RoleAggregateRepository: Send + Sync {
    async fn create(&self, aggregate: &RoleAggregate) -> AppResult<()>;
    async fn save(&self, aggregate: &RoleAggregate) -> AppResult<()>;
    async fn find_by_id(&self, id: &RoleId) -> AppResult<Option<RoleAggregate>>;
    async fn delete_by_id(&self, id: &RoleId) -> AppResult<()>;
}
