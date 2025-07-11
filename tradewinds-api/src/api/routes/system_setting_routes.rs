use crate::api::handlers::system_setting_handler::SystemSettingHandler;
use crate::api::state::AppState;
use axum::{
    Router,
    routing::{get, put},
};
use std::sync::Arc;

pub fn system_setting_routes() -> Router<AppState> {
    Router::new()
        .route("/system/settings/{key}", get(SystemSettingHandler::handle_get_system_setting))
        .route("/system/settings/{key}", put(SystemSettingHandler::handle_set_system_setting))
}
