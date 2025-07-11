use axum::{
    Json,
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::{debug, error, warn};

use tradewinds_common::ApiResponse;
use tradewinds_error::AppError;

pub async fn error_handler(err: AppError) -> Response {
    // 记录详细的错误信息用于调试
    debug!("Handling error in middleware: {:?}", err);

    let (status, error_message) = match &err {
        AppError::Unauthorized(msg) => {
            warn!("Middleware - Unauthorized: {}", msg);
            (StatusCode::UNAUTHORIZED, msg.clone())
        }
        AppError::Forbidden(msg) => {
            warn!("Middleware - Forbidden: {}", msg);
            (StatusCode::FORBIDDEN, msg.clone())
        }
        AppError::NotFound(msg) => {
            warn!("Middleware - Not found: {}", msg);
            (StatusCode::NOT_FOUND, msg.clone())
        }
        AppError::Validation(msg) => {
            warn!("Middleware - Validation error: {}", msg);
            (StatusCode::BAD_REQUEST, msg.clone())
        }
        AppError::Conflict(msg) => {
            warn!("Middleware - Conflict: {}", msg);
            (StatusCode::CONFLICT, msg.clone())
        }
        AppError::Authentication(msg) => {
            warn!("Middleware - Authentication failed: {}", msg);
            (StatusCode::UNAUTHORIZED, msg.clone())
        }
        AppError::DatabaseError(msg) => {
            error!("Middleware - Database error: {}", msg);
            (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
        }
        AppError::System(msg) => {
            error!("Middleware - System error: {}", msg);
            (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
        }
        AppError::Business(msg) => {
            warn!("Middleware - Business error: {}", msg);
            (StatusCode::BAD_REQUEST, msg.clone())
        }
        AppError::Internal(msg) => {
            error!("Middleware - Internal error: {}", msg);
            (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
        }
        AppError::ParseError(msg) => {
            warn!("Middleware - Parse error: {}", msg);
            (StatusCode::BAD_REQUEST, msg.clone())
        }
    };

    // 记录完整错误用于调试
    error!("Error middleware response: status={}, error={:?}", status.as_u16(), err);

    let response = ApiResponse { success: false, message: Some(error_message), data: None::<()> };

    (status, Json(response)).into_response()
}

pub async fn handle_error(error: Box<dyn std::error::Error + Send + Sync>) -> impl IntoResponse {
    let (status, msg) = if let Some(app_error) = error.downcast_ref::<AppError>() {
        match app_error {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    };

    (status, Json(json!({ "error": msg }))).into_response()
}
