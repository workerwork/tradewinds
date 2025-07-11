use std::sync::Arc;

use axum::extract::{Json, Path, Query, State};

#[rustfmt::skip]
use crate::api::{
    controllers::role_controller::RoleController,
    dtos::role_dto::*,
    state::AppState,
};
use crate::api::dtos::{
    AssignPermissionRequest, AssignPermissionResponse, CreateRoleRequest, CreateRoleResponse, DeleteRoleRequest,
    DeleteRoleResponse, GetRoleByIdRequest, GetRoleByIdResponse, GetRoleByNameRequest, GetRoleByNameResponse,
    GetRolePermissionsRequest, GetRolePermissionsResponse, ListRolesRequest, ListRolesResponse,
    RevokePermissionRequest, RevokePermissionResponse, UpdateRoleRequest, UpdateRoleResponse,
};
use tradewinds_common::ApiResponse;
use tradewinds_error::AppResult;

pub struct RoleHandler {
    role_controller: Arc<RoleController>,
}

impl RoleHandler {

    /// 创建角色
    pub async fn handle_create_role(
        State(state): State<AppState>,
        Json(req): Json<CreateRoleRequest>,
    ) -> AppResult<Json<ApiResponse<CreateRoleResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.role_controller.create_role(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 更新角色
    pub async fn handle_update_role(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Json(mut req): Json<UpdateRoleRequest>,
    ) -> AppResult<Json<ApiResponse<UpdateRoleResponse>>> {
        // 从路径参数设置id
        req.id = id;
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.role_controller.update_role(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 删除角色
    pub async fn handle_delete_role(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<DeleteRoleResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let req = DeleteRoleRequest { id };
        let resp = state.role_controller.delete_role(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 分配权限
    pub async fn handle_assign_permission(
        State(state): State<AppState>,
        Json(req): Json<AssignPermissionRequest>,
    ) -> AppResult<Json<ApiResponse<AssignPermissionResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let _ = state.role_controller.assign_permission(actor_id, req).await?;
        Ok(Json(ApiResponse::success(AssignPermissionResponse)))
    }

    /// 撤销权限
    pub async fn handle_revoke_permission(
        State(state): State<AppState>,
        Json(req): Json<RevokePermissionRequest>,
    ) -> AppResult<Json<ApiResponse<RevokePermissionResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let _ = state.role_controller.revoke_permission(actor_id, req).await?;
        Ok(Json(ApiResponse::success(RevokePermissionResponse)))
    }

    /// 获取角色
    pub async fn handle_get_role(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<GetRoleByIdResponse>>> {
        let req = GetRoleByIdRequest { id };
        let resp = state.role_controller.get_role_by_id(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据角色名获取角色
    pub async fn handle_get_role_by_name(
        State(state): State<AppState>,
        Json(req): Json<GetRoleByNameRequest>,
    ) -> AppResult<Json<ApiResponse<GetRoleByNameResponse>>> {
        let resp = state.role_controller.get_role_by_name(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取角色权限
    pub async fn handle_get_role_permissions(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<GetRolePermissionsResponse>>> {
        let req = GetRolePermissionsRequest { role_id: id };
        let resp = state.role_controller.get_role_permissions(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取角色列表
    pub async fn handle_list_roles(
        State(state): State<AppState>,
        Query(req): Query<ListRolesRequest>,
    ) -> AppResult<Json<ApiResponse<ListRolesResponse>>> {
        let resp = state.role_controller.list_roles(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }
}
