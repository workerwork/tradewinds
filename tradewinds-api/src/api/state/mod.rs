//! 状态管理
//! 包含所有控制器的引用
//! 用于在路由中共享状态
//! 用于在中间件中共享状态
//! 用于在控制器中共享状态
//! 用于在服务中共享状态
//! 用于在模型中共享状态
//! 用于在存储中共享状态
//! 用于在配置中共享状态
//! 用于在测试中共享状态
//! 用于在文档中共享状态
//! 用于在日志中共享状态
//! 用于在错误处理中共享状态
//! 用于在性能优化中共享状态
//! 用于在安全中共享状态
//! 用于在性能优化中共享状态

use std::sync::Arc;

use tradewinds_domain::services::auth::token_service::TokenService;

#[rustfmt::skip]
use crate::api::controllers::{
    auth_controller::AuthController,
    role_controller::RoleController,
    permission_controller::PermissionController,
    user_controller::UserController,
    system_setting_controller::SystemSettingController,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_controller: Arc<AuthController>,
    pub user_controller: Arc<UserController>,
    pub role_controller: Arc<RoleController>,
    pub permission_controller: Arc<PermissionController>,
    pub system_setting_controller: Arc<SystemSettingController>,
    // FIXME: 这里需要一个更好的方式来管理 token_service
    // 因为 token_service 需要被多个控制器共享，所以需要一个更好的方式来管理它
    // 目前这个方式是临时的，后续需要优化
    pub token_service: Arc<dyn TokenService>,
}

impl AppState {
    pub fn new(
        auth_controller: AuthController,
        user_controller: UserController,
        role_controller: RoleController,
        permission_controller: PermissionController,
        system_setting_controller: SystemSettingController,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            auth_controller: Arc::new(auth_controller),
            user_controller: Arc::new(user_controller),
            role_controller: Arc::new(role_controller),
            permission_controller: Arc::new(permission_controller),
            system_setting_controller: Arc::new(system_setting_controller),
            token_service,
        }
    }
}

// unsafe impl Send for AppState {}
// unsafe impl Sync for AppState {}
