use axum::{Json, extract::State, http::HeaderMap};
use std::sync::Arc;

use tradewinds_common::{ApiResponse, utils::get_current_user_token};
use tradewinds_error::AppResult;

#[rustfmt::skip]
use crate::api::{
    dtos::{
        LoginRequest, LoginResponse, 
        LogoutRequest, LogoutResponse, 
        RegisterRequest, RegisterResponse,
        ChangePasswordRequest, ChangePasswordResponse, 
        GetCurrentUserRequest, GetCurrentUserResponse,
        GetUserMenusRequest, GetUserMenusResponse,
        GetSuperAdminDashboardRequest, GetSuperAdminDashboardResponse, 
    },
    AppState, AuthController,
};

/// 处理用户登录、注册、修改密码、获取当前用户等操作的请求
pub struct AuthHandler {
    auth_controller: Arc<AuthController>,
}

impl AuthHandler {
    /// 用户登录
    pub async fn handle_login(
        State(state): State<AppState>,
        Json(req): Json<LoginRequest>,
    ) -> AppResult<Json<ApiResponse<LoginResponse>>> {
        let resp = state.auth_controller.login(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 用户登出
    pub async fn handle_logout(
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> AppResult<Json<ApiResponse<LogoutResponse>>> {
        let token = get_current_user_token(&headers).await?;
        let req = LogoutRequest { token };
        let resp = state.auth_controller.logout(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 用户注册
    pub async fn handle_register(
        State(state): State<AppState>,
        Json(req): Json<RegisterRequest>,
    ) -> AppResult<Json<ApiResponse<RegisterResponse>>> {
        let resp = state.auth_controller.register(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 修改密码
    pub async fn handle_change_password(
        State(state): State<AppState>,
        headers: HeaderMap,
        Json(req): Json<ChangePasswordRequest>,
    ) -> AppResult<Json<ApiResponse<ChangePasswordResponse>>> {
        let token = get_current_user_token(&headers).await?;
        let resp = state.auth_controller.change_password(token, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取当前用户
    pub async fn handle_get_current_user(
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> AppResult<Json<ApiResponse<GetCurrentUserResponse>>> {
        let token = get_current_user_token(&headers).await?;
        let req = GetCurrentUserRequest { token };
        let resp = state.auth_controller.get_current_user(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取用户菜单权限
    pub async fn handle_get_user_menus(
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> AppResult<Json<ApiResponse<GetUserMenusResponse>>> {
        let token = get_current_user_token(&headers).await?;
        let req = GetUserMenusRequest { token };
        let resp = state.auth_controller.get_user_menus(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取超级管理员仪表盘数据
    pub async fn handle_get_super_admin_dashboard(
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> AppResult<Json<ApiResponse<GetSuperAdminDashboardResponse>>> {
        let token = get_current_user_token(&headers).await?;
        let req = GetSuperAdminDashboardRequest { token };
        let resp = state.auth_controller.get_super_admin_dashboard(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }
}
