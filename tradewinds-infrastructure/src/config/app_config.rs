use std::env;

use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct AppConfig {
    // 数据库配置
    pub database_url: String,
    // Redis配置
    pub redis_url: String,
    // 认证配置
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    // 服务配置
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").map_err(|_| AppError::System("DATABASE_URL not set".to_string()))?,
            redis_url: env::var("REDIS_URL").map_err(|_| AppError::System("REDIS_URL not set".to_string()))?,
            jwt_secret: env::var("JWT_SECRET").map_err(|_| AppError::System("JWT_SECRET not set".to_string()))?,
            jwt_expiration: env::var("JWT_EXPIRATION")
                .map_err(|_| AppError::System("JWT_EXPIRATION not set".to_string()))?
                .parse()
                .map_err(|_| AppError::System("JWT_EXPIRATION must be a number".to_string()))?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| AppError::System("SERVER_PORT must be a number".to_string()))?,
        })
    }
}
