use async_trait::async_trait;
use tradewinds_error::AppResult;

/// 密码加密服务 trait
#[async_trait]
pub trait PasswordService: Send + Sync {
    /// 加密密码
    async fn hash(&self, raw: &str) -> AppResult<String>;

    /// 校验密码是否匹配
    async fn verify(&self, hashed: &str, raw: &str) -> AppResult<bool>;

    /// 校验密码强度
    async fn validate_password_strength(&self, password: &str) -> AppResult<()>;
}
