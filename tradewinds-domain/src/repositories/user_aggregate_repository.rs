use crate::aggregates::user_aggregate::UserAggregate;
use crate::value_objects::user::UserId;
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait UserAggregateRepository: Send + Sync {
    async fn find_by_id(&self, user_id: &UserId) -> AppResult<Option<UserAggregate>>;
    async fn save(&self, aggregate: &UserAggregate) -> AppResult<()>;
    async fn create(&self, aggregate: &UserAggregate) -> AppResult<()>;
    async fn delete_by_id(&self, id: &UserId) -> AppResult<()>;
}
