use crate::config::AppConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tradewinds_domain::repositories::TokenBlacklistRepository;
use tradewinds_domain::services::auth::{TokenService, token_service::TokenClaims};
use tradewinds_domain::value_objects::{auth::auth_token::Token, user::UserId};
use tradewinds_error::{AppError, AppResult};


/// JWT Payload（Token Claims）
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String, // user_id
    exp: i64,
}

/// JWT 实现    
#[derive(Clone)]
pub struct JwtTokenService<B: TokenBlacklistRepository> {
    config: AppConfig,
    blacklist_repo: B,
}

impl<B: TokenBlacklistRepository> JwtTokenService<B> {
    pub fn new(config: AppConfig, blacklist_repo: B) -> Self {
        Self { config, blacklist_repo }
    }
}

#[async_trait::async_trait]
impl<B: TokenBlacklistRepository + Clone + 'static> TokenService for JwtTokenService<B> {
    async fn generate(&self, user_id: &UserId) -> AppResult<Token> {
        let claims = Claims {
            sub: user_id.value().to_string(),
            exp: (Utc::now() + Duration::minutes(self.config.jwt_expiration)).timestamp(),
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.config.jwt_secret.as_ref()))
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(Token::new(token)?)
    }

    async fn validate(&self, token: &Token) -> AppResult<TokenClaims> {
        // 1. 首先检查Token是否在黑名单中
        if self.blacklist_repo.is_blacklisted(token).await? {
            return Err(AppError::Authentication("Token has been revoked".to_string()));
        }

        // 2. 验证JWT Token格式和签名
        let token_data = decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))?;

        Ok(TokenClaims { user_id: UserId::new(token_data.claims.sub)?, exp: token_data.claims.exp })
    }

    async fn revoke(&self, token: &Token) -> AppResult<()> {
        // 直接解码JWT Token以获取用户ID和过期时间（不检查黑名单，避免递归）
        let token_data = decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))?;

        let user_id = UserId::new(token_data.claims.sub)?;

        // 将Token添加到黑名单
        self.blacklist_repo.add(token, &user_id, token_data.claims.exp).await
    }

    async fn get_user_id_from_token(&self, token: &Token) -> AppResult<UserId> {
        let token_data = decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::Authentication(format!("Invalid token: {}", e)))?;
        Ok(UserId::new(token_data.claims.sub)?)
    }
}
