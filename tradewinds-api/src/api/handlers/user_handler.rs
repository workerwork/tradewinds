use std::sync::Arc;

use axum::extract::{Json, Path, Query, State};
use axum::http::HeaderMap;

use crate::api::dtos::{
    AssignRoleRequest, AssignRoleResponse, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetSystemSettingRequest, GetUserByEmailRequest, GetUserByEmailResponse, GetUserByIdRequest,
    GetUserByIdResponse, GetUserByUsernameRequest, GetUserByUsernameResponse, GetUserPermissionsRequest,
    GetUserPermissionsResponse, GetUserRolesRequest, GetUserRolesResponse, ListUsersRequest, ListUsersResponse,
    ResetPasswordRequest, ResetPasswordResponse, RevokeRoleRequest, RevokeRoleResponse, UpdateCurrentUserRequest,
    UpdateUserRequest, UpdateUserResponse,
};
use crate::api::{AppState, UserController};
use tradewinds_common::ApiResponse;
use tradewinds_common::get_current_user_token;
use tradewinds_domain::value_objects::Token;
use tradewinds_error::AppResult;

pub struct UserHandler {
    user_controller: Arc<UserController>,
}

impl UserHandler {

    /// 创建用户
    pub async fn handle_create_user(
        State(state): State<AppState>,
        Json(req): Json<CreateUserRequest>,
    ) -> AppResult<Json<ApiResponse<CreateUserResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.user_controller.create_user(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 更新用户
    pub async fn handle_update_user(
        State(state): State<AppState>,
        Json(req): Json<UpdateUserRequest>,
    ) -> AppResult<Json<ApiResponse<UpdateUserResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.user_controller.update_user(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 更新当前用户
    pub async fn handle_update_current_user(
        State(state): State<AppState>,
        headers: HeaderMap,
        Json(mut req): Json<UpdateCurrentUserRequest>,
    ) -> AppResult<Json<ApiResponse<UpdateUserResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let token = get_current_user_token(&headers).await?;
        let token = Token::new(token)?;
        let user_id = state.token_service.get_user_id_from_token(&token).await?.to_string();
        let req = UpdateUserRequest {
            id: user_id.clone(),
            real_name: req.real_name,
            phone: req.phone,
            avatar: req.avatar,
            email: req.email,
            role_ids: None,
            status: None,
        };
        let resp = state.user_controller.update_user(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 删除用户
    pub async fn handle_delete_user(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<DeleteUserResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let req = DeleteUserRequest { id };
        let resp = state.user_controller.delete_user(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 重置密码
    pub async fn handle_reset_password(
        State(state): State<AppState>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ApiResponse<ResetPasswordResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.user_controller.reset_password(actor_id, id).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 分配角色
    pub async fn handle_assign_role(
        State(state): State<AppState>,
        Json(req): Json<AssignRoleRequest>,
    ) -> AppResult<Json<ApiResponse<AssignRoleResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.user_controller.assign_role(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 撤销角色
    pub async fn handle_revoke_role(
        State(state): State<AppState>,
        Json(req): Json<RevokeRoleRequest>,
    ) -> AppResult<Json<ApiResponse<RevokeRoleResponse>>> {
        let actor_id = "00000000-0000-0000-0000-000000000000".to_string(); // FIXME
        let resp = state.user_controller.revoke_role(actor_id, req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取用户
    pub async fn handle_get_user(
        State(state): State<AppState>,
        Json(req): Json<GetUserByIdRequest>,
    ) -> AppResult<Json<ApiResponse<GetUserByIdResponse>>> {
        let resp = state.user_controller.get_user_by_id(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据用户名获取用户
    pub async fn handle_get_user_by_username(
        State(state): State<AppState>,
        Json(req): Json<GetUserByUsernameRequest>,
    ) -> AppResult<Json<ApiResponse<GetUserByUsernameResponse>>> {
        let resp = state.user_controller.get_user_by_username(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 根据邮箱获取用户
    pub async fn handle_get_user_by_email(
        State(state): State<AppState>,
        Json(req): Json<GetUserByEmailRequest>,
    ) -> AppResult<Json<ApiResponse<GetUserByEmailResponse>>> {
        let resp = state.user_controller.get_user_by_email(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取用户角色
    pub async fn handle_get_user_roles(
        State(state): State<AppState>,
        Json(req): Json<GetUserRolesRequest>,
    ) -> AppResult<Json<ApiResponse<GetUserRolesResponse>>> {
        let resp = state.user_controller.get_user_roles(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取用户权限
    pub async fn handle_get_user_permissions(
        State(state): State<AppState>,
        Json(req): Json<GetUserPermissionsRequest>,
    ) -> AppResult<Json<ApiResponse<GetUserPermissionsResponse>>> {
        let resp = state.user_controller.get_user_permissions(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 获取用户列表
    pub async fn handle_list_users(
        State(state): State<AppState>,
        Query(req): Query<ListUsersRequest>,
    ) -> AppResult<Json<ApiResponse<ListUsersResponse>>> {
        let resp = state.user_controller.list_users(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }
}
