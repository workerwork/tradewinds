use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

#[rustfmt::skip]
use crate::api::{
    handlers::user_handler::UserHandler, 
    state::AppState
};

pub fn user_routes() -> Router<AppState> {
    // 创建路由
    Router::new()
        .route("/system/users", get(UserHandler::handle_list_users))
        .route("/system/users", post(UserHandler::handle_create_user))
        .route("/system/users/me", put(UserHandler::handle_update_current_user))
        .route("/system/users/{id}", get(UserHandler::handle_get_user))
        .route("/system/users/{id}", put(UserHandler::handle_update_user))
        .route("/system/users/{id}", patch(UserHandler::handle_update_user))
        .route("/system/users/{id}", delete(UserHandler::handle_delete_user))
        .route("/system/users/{id}/reset-password", post(UserHandler::handle_reset_password))
        .route("/system/users/{id}/roles", get(UserHandler::handle_get_user_roles))
        .route("/system/users/{id}/permissions", get(UserHandler::handle_get_user_permissions))
}
