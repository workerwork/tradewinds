use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

#[rustfmt::skip]
use crate::api::{
    handlers::role_handler::RoleHandler, 
    state::AppState
};

pub fn role_routes() -> Router<AppState> {
    // 创建路由
    Router::new()
        .route("/system/roles", get(RoleHandler::handle_list_roles))
        .route("/system/roles", post(RoleHandler::handle_create_role))
        .route("/system/roles/{id}", get(RoleHandler::handle_get_role))
        .route("/system/roles/{id}", put(RoleHandler::handle_update_role))
        .route("/system/roles/{id}", patch(RoleHandler::handle_update_role))
        .route("/system/roles/{id}", delete(RoleHandler::handle_delete_role))
        .route("/system/roles/{id}/permissions", get(RoleHandler::handle_get_role_permissions))
}
