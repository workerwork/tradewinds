use std::sync::Arc;

use axum::extract::{Json, State};

#[rustfmt::skip]
use crate::api::{
    controllers::system_setting_controller::SystemSettingController,
    dtos::system_setting_dto::*,
    state::AppState,
};
use crate::api::dtos::{
    GetSystemSettingRequest, GetSystemSettingResponse, SetSystemSettingRequest, SetSystemSettingResponse,
};
use tradewinds_common::ApiResponse;
use tradewinds_error::AppResult;

pub struct SystemSettingHandler {
    system_setting_controller: Arc<SystemSettingController>,
}

impl SystemSettingHandler {

    /// 获取系统设置
    pub async fn handle_get_system_setting(
        State(state): State<AppState>,
        Json(req): Json<GetSystemSettingRequest>,
    ) -> AppResult<Json<ApiResponse<GetSystemSettingResponse>>> {
        let resp = state.system_setting_controller.get_by_key(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }

    /// 设置系统设置
    pub async fn handle_set_system_setting(
        State(state): State<AppState>,
        Json(req): Json<SetSystemSettingRequest>,
    ) -> AppResult<Json<ApiResponse<SetSystemSettingResponse>>> {
        let resp = state.system_setting_controller.set_value(req).await?;
        Ok(Json(ApiResponse::success(resp)))
    }
}
