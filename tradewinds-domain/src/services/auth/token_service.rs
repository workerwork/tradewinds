//! 令牌服务，生成和验证令牌
use crate::value_objects::Token;
use crate::value_objects::user::UserId;
use tradewinds_error::AppResult;

pub struct TokenClaims {
    pub user_id: UserId,
    pub exp: i64,
}

#[async_trait::async_trait]
pub trait TokenService: Send + Sync + 'static {
    async fn generate(&self, user_id: &UserId) -> AppResult<Token>;
    async fn validate(&self, token: &Token) -> AppResult<TokenClaims>;
    async fn revoke(&self, token: &Token) -> AppResult<()>;
    async fn get_user_id_from_token(&self, token: &Token) -> AppResult<UserId>;
}
