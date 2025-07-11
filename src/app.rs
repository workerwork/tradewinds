// 标准库 & 三方库
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

// Axum
use axum::{Router, middleware};

// 错误与配置
use tradewinds_error::{AppError, AppResult};
use tradewinds_infrastructure::config::AppConfig;
use tradewinds_infrastructure::dependency_injection::init_application_service;

// API 层
use tradewinds_api::api::controllers::{
    AuthController, PermissionController, RoleController, SystemSettingController, UserController,
};
use tradewinds_api::api::middlewares::security;
use tradewinds_api::api::routes::{auth_routes, permission_routes, role_routes, user_routes};
use tradewinds_api::api::state::AppState;

// 领域服务
use tradewinds_domain::services::auth::token_service::TokenService;

// Application interfaces
use tradewinds_application::interfaces::{
    IAuthService, IPermissionService, IRoleService, ISystemSettingService, IUserService,
};

pub struct App {
    config: AppConfig,
    router: Router,
}

impl App {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        // 初始化服务
        let (auth_service, user_service, role_service, permission_service, token_service, system_setting_service): (
            Arc<dyn IAuthService>,
            Arc<dyn IUserService>,
            Arc<dyn IRoleService>,
            Arc<dyn IPermissionService>,
            Arc<dyn TokenService>,
            Arc<dyn ISystemSettingService>,
        ) = init_application_service(&config).await.map_err(|e| AppError::System(e.to_string()))?;

        let system_setting_controller = SystemSettingController::assemble(system_setting_service.clone());
        let auth_controller = AuthController::assemble(auth_service.clone());
        let user_controller = UserController::assemble(user_service.clone(), system_setting_service.clone());
        let role_controller = RoleController::assemble(role_service.clone());
        let permission_controller = PermissionController::assemble(permission_service.clone());

        // 创建共享状态（含认证服务）
        let state = AppState::new(
            auth_controller,
            user_controller,
            role_controller,
            permission_controller,
            system_setting_controller,
            token_service,
        );

        // 构建 router，注入状态
        let protected_routes = Router::new()
            .merge(user_routes::user_routes())
            .merge(role_routes::role_routes())
            .merge(permission_routes::permission_routes())
            .layer(middleware::from_fn_with_state(state.clone(), security::auth));

        let router = Router::new().merge(auth_routes::auth_routes()).merge(protected_routes).with_state(state);

        Ok(Self { config, router })
    }

    pub async fn run(&self) -> AppResult<()> {
        let addr = format!("{}:{}", self.config.server_host, self.config.server_port).parse::<SocketAddr>().unwrap();
        info!("Server running on http://{}", addr);
        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, self.router.clone().into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

        Ok(())
    }
}
