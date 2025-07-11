use crate::api::dtos::auth_dto::GetCurrentUserRequest;
use crate::api::state::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response},
    middleware::Next,
};
use tradewinds_common::get_current_user_token;
use tradewinds_error::AppError;

pub async fn auth(State(state): State<AppState>, req: Request<Body>, next: Next) -> Result<Response<Body>, AppError> {
    let auth_controller = &state.auth_controller;

    // 1. 提取token（需要 await）
    let token = get_current_user_token(req.headers()).await?;

    // 2. 校验token
    let get_user_req = GetCurrentUserRequest { token: token.clone() };
    let user_result = auth_controller.get_current_user(get_user_req).await;
    if user_result.is_err() {
        return Err(AppError::Unauthorized("Invalid or expired token".to_string()));
    }

    // 3. 可选：将用户信息插入extensions，后续handler可用
    // let user_info = user_result.unwrap();
    // req.extensions_mut().insert(user_info.user.user.id.clone());

    // 4. 放行
    Ok(next.run(req).await)
}
