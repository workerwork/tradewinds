use axum::{
    Router,
    routing::{get, post},
};

use crate::api::{handlers::auth_handler::AuthHandler, state::AppState};

/// 认证相关路由
///
/// - /auth/login 登录
/// - /auth/logout 登出
/// - /auth/register 注册
/// - /auth/change-password 修改密码
/// - /auth/me 获取当前用户信息
/// - /auth/menus 获取当前用户菜单
/// - /auth/super-admin/dashboard 获取超级管理员仪表盘
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        // 用户登录
        .route("/auth/login", post(AuthHandler::handle_login))
        // 用户登出
        .route("/auth/logout", post(AuthHandler::handle_logout))
        // 用户注册
        .route("/auth/register", post(AuthHandler::handle_register))
        // 修改密码
        .route("/auth/change-password", post(AuthHandler::handle_change_password))
        // 获取当前用户信息
        .route("/auth/me", get(AuthHandler::handle_get_current_user))
        // 获取当前用户菜单
        .route("/auth/menus", get(AuthHandler::handle_get_user_menus))
        // 获取超级管理员仪表盘
        .route("/auth/super-admin/dashboard", get(AuthHandler::handle_get_super_admin_dashboard))
}
