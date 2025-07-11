use axum::http::HeaderMap;
use serde::{Deserialize, Deserializer};
use tradewinds_error::{AppError, AppResult};

pub async fn get_current_user_token(headers: &HeaderMap) -> AppResult<String> {
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AppError::Authentication("Missing or invalid authorization header".to_string()))?;
    Ok(token.to_string())
}

/// 通用：空字符串自动转 None
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.and_then(|s| if s.trim().is_empty() { None } else { Some(s) }))
}
