use std::sync::Arc;

use axum::extract::{Json, Path, Query, State};

#[rustfmt::skip]
use crate::api::{
    controllers::permission_controller::PermissionController,
    dtos::permission_dto::*,
    state::AppState,
};
use crate::api::dtos::permission_dto::PermissionTreeResponse;
use crate::api::dtos::{
    CreatePermissionRequest, CreatePermissionResponse, DeletePermissionRequest, DeletePermissionResponse,
    GetPermissionByCodeRequest, GetPermissionByCodeResponse, GetPermissionByIdRequest, GetPermissionByIdResponse,
    GetPermissionByNameRequest, GetPermissionByNameResponse, ListPermissionsByParentIdRequest,
    ListPermissionsByParentIdResponse, ListPermissionsByTypeRequest, ListPermissionsByTypeResponse,
    ListPermissionsRequest, ListPermissionsResponse, UpdatePermissionRequest, UpdatePermissionResponse,
};
use crate::api::mappers::permission_mapper::to_permission_tree_response;
use tradewinds_common::ApiResponse;
use tradewinds_error::AppResult;

pub struct PermissionHandler {
    permission_controller: Arc<PermissionController>,
}

impl PermissionHandler {
    /// 创建权限
    pub async fn handle_create_permission(
        State(state): State<AppState>,
        Json(req): Json<CreatePermissionRequest>,
    ) -> AppResult<Json<ApiResponse<CreatePermissionResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string();
        let resp = state.permission_controller.create_permission(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 更新权限
    pub async fn handle_update_permission(
        State(state): State<AppState>,
        Json(req): Json<UpdatePermissionRequest>,
    ) -> AppResult<Json<ApiResponse<UpdatePermissionResponse>>> {
        // FIXME: Use a real actor_id from token
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string();
        let resp = state.permission_controller.update_permission(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 删除权限
    pub async fn handle_delete_permission(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<DeletePermissionResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string();
        let req = DeletePermissionRequest { id };
        let resp = state.permission_controller.delete_permission(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取权限
    pub async fn handle_get_permission(
        State(state): State<AppState>,
        Json(req): Json<GetPermissionByIdRequest>,
    ) -> AppResult<Json<ApiResponse<GetPermissionByIdResponse>>> {
        let resp = state.permission_controller.get_permission_by_id(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据权限码获取权限
    pub async fn handle_get_permission_by_code(
        State(state): State<AppState>,
        Json(req): Json<GetPermissionByCodeRequest>,
    ) -> AppResult<Json<ApiResponse<GetPermissionByCodeResponse>>> {
        let resp = state.permission_controller.get_permission_by_code(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据权限名获取权限
    pub async fn handle_get_permission_by_name(
        State(state): State<AppState>,
        Json(req): Json<GetPermissionByNameRequest>,
    ) -> AppResult<Json<ApiResponse<GetPermissionByNameResponse>>> {
        let resp = state.permission_controller.get_permission_by_name(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取权限列表
    pub async fn handle_list_permissions(
        State(state): State<AppState>,
        Query(query): Query<ListPermissionsRequest>,
    ) -> AppResult<Json<ApiResponse<ListPermissionsResponse>>> {
        let resp = state.permission_controller.list_permissions(query).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据权限类型获取权限列表
    pub async fn handle_list_permissions_by_type(
        State(state): State<AppState>,
        Query(query): Query<ListPermissionsByTypeRequest>,
    ) -> AppResult<Json<ApiResponse<ListPermissionsByTypeResponse>>> {
        let resp = state.permission_controller.list_permissions_by_type(query).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据父权限ID获取权限列表
    pub async fn handle_list_permissions_by_parent_id(
        State(state): State<AppState>,
        Query(query): Query<ListPermissionsByParentIdRequest>,
    ) -> AppResult<Json<ApiResponse<ListPermissionsByParentIdResponse>>> {
        let resp = state.permission_controller.list_permissions_by_parent_id(query).await?;
        Ok(Json(ApiResponse::success(resp)))
    }
}

pub async fn handle_get_permission_tree(State(state): State<AppState>) -> AppResult<Json<Vec<PermissionTreeResponse>>> {
    let permissions = state.permission_controller.list_all_permissions().await?;
    let flat: Vec<PermissionTreeResponse> = permissions.into_iter().map(PermissionTreeResponse::from).collect();
    Ok(Json(PermissionTreeResponse::build_tree(flat)))
}
