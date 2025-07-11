use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait Event: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn to_json(&self) -> AppResult<String>;
}
