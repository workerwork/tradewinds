use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use tokio::task::spawn_blocking;
use tradewinds_domain::services::auth::PasswordService;
use tradewinds_domain::specifications::Specification;
use tradewinds_domain::specifications::password_strength_specification::PasswordStrengthSpecification;
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct BcryptPasswordService;

impl BcryptPasswordService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PasswordService for BcryptPasswordService {
    async fn hash(&self, raw: &str) -> AppResult<String> {
        let raw = raw.to_owned();
        let hashed = spawn_blocking(move || hash(&raw, DEFAULT_COST))
            .await
            .map_err(|e| AppError::Internal(format!("Join error: {}", e)))?;
        hashed.map_err(|e| AppError::Internal(format!("Hash failed: {}", e)))
    }

    async fn verify(&self, hashed: &str, raw: &str) -> AppResult<bool> {
        let hashed = hashed.to_owned();
        let raw = raw.to_owned();
        let result = spawn_blocking(move || verify(&raw, &hashed))
            .await
            .map_err(|e| AppError::Internal(format!("Join error: {}", e)))?;
        result.map_err(|e| AppError::Internal(format!("Verify failed: {}", e)))
    }

    async fn validate_password_strength(&self, password: &str) -> AppResult<()> {
        let specification = PasswordStrengthSpecification::new(8, false, false, false, false);
        if !specification.is_satisfied_by(password) {
            return Err(AppError::Validation(specification.message()));
        }
        Ok(())
    }
}
