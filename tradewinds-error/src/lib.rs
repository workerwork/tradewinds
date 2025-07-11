use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bcrypt::BcryptError;
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};

#[derive(Debug, Error)]
pub enum AppError {

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Business error: {0}")]
    Business(String),

    #[error("System error: {0}")]
    System(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::System(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::System(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::System(format!("JSON error: {}", error))
    }
}

impl From<redis::RedisError> for AppError {
    fn from(error: redis::RedisError) -> Self {
        AppError::System(format!("Redis error: {}", error))
    }
}

impl From<BcryptError> for AppError {
    fn from(err: BcryptError) -> Self {
        AppError::System(format!("Password encryption error: {}", err))
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        AppError::ParseError(error.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error.to_string())
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 根据错误严重程度记录不同级别的日志
        let (status, error_message) = match &self {
            AppError::Unauthorized(msg) => {
                warn!("Unauthorized access: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            AppError::Forbidden(msg) => {
                warn!("Forbidden access: {}", msg);
                (StatusCode::FORBIDDEN, msg.clone())
            }
            AppError::NotFound(msg) => {
                warn!("Resource not found: {}", msg);
                (StatusCode::NOT_FOUND, msg.clone())
            }
            AppError::Validation(msg) => {
                warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::Conflict(msg) => {
                warn!("Conflict error: {}", msg);
                (StatusCode::CONFLICT, msg.clone())
            }
            AppError::Authentication(msg) => {
                warn!("Authentication failed: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            AppError::System(msg) => {
                error!("System error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::Business(msg) => {
                warn!("Business logic error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::Internal(msg) => {
                error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::ParseError(msg) => {
                warn!("Parse error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::DatabaseError(msg) => {
                error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
        };

        // 记录完整的错误信息用于调试
        error!("Error response: status={}, error={:?}", status.as_u16(), self);

        let error_type = match status {
            StatusCode::UNAUTHORIZED => "auth_error",
            StatusCode::FORBIDDEN => "forbidden",
            StatusCode::NOT_FOUND => "not_found",
            StatusCode::BAD_REQUEST => "validation_error",
            StatusCode::CONFLICT => "conflict",
            _ => "system_error",
        };

        let body = Json(json!({
            "error": {
                "code": status.as_u16(),
                "message": error_message,
                "type": error_type
            }
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

// 调试辅助宏
#[macro_export]
macro_rules! debug_error {
    ($error:expr, $context:expr) => {{
        tracing::error!("Debug Error - Context: {}, Error: {:?}", $context, $error);
        $error
    }};
}

#[macro_export]
macro_rules! trace_result {
    ($result:expr, $operation:expr) => {
        match &$result {
            Ok(val) => {
                tracing::debug!("Operation '{}' succeeded", $operation);
                $result
            }
            Err(err) => {
                tracing::error!("Operation '{}' failed: {:?}", $operation, err);
                $result
            }
        }
    };
}
