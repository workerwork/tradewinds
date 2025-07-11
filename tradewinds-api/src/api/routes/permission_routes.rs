use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::api::{
    handlers::permission_handler::{PermissionHandler, handle_get_permission_tree},
    state::AppState,
};

/// 权限管理相关路由
///
/// - /system/permissions 权限创建、列表
/// - /system/permissions/{id} 权限更新、删除
/// - /system/permissions/code/{code} 根据权限码获取权限
/// - /system/permissions/tree 获取权限树
pub fn permission_routes() -> Router<AppState> {
    Router::new()
        // 创建权限
        .route("/system/permissions", post(PermissionHandler::handle_create_permission))
        // 更新权限
        .route("/system/permissions/{id}", put(PermissionHandler::handle_update_permission))
        // 局部更新权限
        .route("/system/permissions/{id}", patch(PermissionHandler::handle_update_permission))
        // 删除权限
        .route("/system/permissions/{id}", delete(PermissionHandler::handle_delete_permission))
        // 获取权限列表
        .route("/system/permissions", get(PermissionHandler::handle_list_permissions))
        // 根据权限码获取权限
        .route("/system/permissions/code/{code}", get(PermissionHandler::handle_get_permission_by_code))
        // 获取权限树
        .route("/system/permissions/tree", get(handle_get_permission_tree))
}
