use crate::value_objects::auth::auth_token::Token;
use crate::value_objects::user::UserId;
use tradewinds_error::AppResult;

#[async_trait::async_trait]
pub trait TokenBlacklistRepository: Send + Sync {
    async fn add(&self, token: &Token, user_id: &UserId, expires_at: i64) -> AppResult<()>;
    async fn is_blacklisted(&self, token: &Token) -> AppResult<bool>;
    async fn cleanup(&self) -> AppResult<()>;
}
