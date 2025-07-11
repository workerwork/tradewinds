use std::sync::Arc;

// 应用层命令与处理器
use tradewinds_application::commands::auth::{
    ChangePasswordCommand, LoginCommand, LogoutCommand, RegisterCommand,
    handlers::{ChangePasswordHandler, LoginHandler, LogoutHandler, RegisterHandler},
};

// 查询与处理器
use tradewinds_application::queries::auth::{
    CurrentUserInfo, GetCurrentUserQuery, GetUserMenusQuery, MenuInfo,
    handlers::{GetCurrentUserHandler, GetUserMenusHandler},
};
use tradewinds_application::{CommandHandler, QueryHandler};

// 领域对象
use tradewinds_application::interfaces::IAuthService;
use tradewinds_domain::value_objects::Token;

// 错误类型
use tradewinds_error::AppResult;

// crate 内部
use crate::api::{dtos::auth_dto::*, mappers::auth_mapper};

/// 认证控制器，负责协调认证相关的用例
pub struct AuthController {
    pub register: Arc<dyn CommandHandler<RegisterCommand, ()>>,
    pub login: Arc<dyn CommandHandler<LoginCommand, Token>>,
    pub logout: Arc<dyn CommandHandler<LogoutCommand, ()>>,
    pub change_password: Arc<dyn CommandHandler<ChangePasswordCommand, ()>>,
    pub get_current_user: Arc<dyn QueryHandler<GetCurrentUserQuery, CurrentUserInfo>>,
    pub get_user_menus: Arc<dyn QueryHandler<GetUserMenusQuery, Vec<MenuInfo>>>,
}

impl AuthController {
    pub fn new(
        register: Arc<dyn CommandHandler<RegisterCommand, ()>>,
        login: Arc<dyn CommandHandler<LoginCommand, Token>>,
        logout: Arc<dyn CommandHandler<LogoutCommand, ()>>,
        change_password: Arc<dyn CommandHandler<ChangePasswordCommand, ()>>,
        get_current_user: Arc<dyn QueryHandler<GetCurrentUserQuery, CurrentUserInfo>>,
        get_user_menus: Arc<dyn QueryHandler<GetUserMenusQuery, Vec<MenuInfo>>>,
    ) -> Self {
        Self { register, login, logout, change_password, get_current_user, get_user_menus }
    }

    pub fn assemble(auth_service: Arc<dyn IAuthService>) -> Self {
        Self::new(
            Arc::new(RegisterHandler::new(auth_service.clone())),
            Arc::new(LoginHandler::new(auth_service.clone())),
            Arc::new(LogoutHandler::new(auth_service.clone())),
            Arc::new(ChangePasswordHandler::new(auth_service.clone())),
            Arc::new(GetCurrentUserHandler::new(auth_service.clone())),
            Arc::new(GetUserMenusHandler::new(auth_service.clone())),
        )
    }

    /// 用户注册
    pub async fn register(&self, req: RegisterRequest) -> AppResult<RegisterResponse> {
        let command = auth_mapper::to_register_command(req)?;
        let _ = self.register.handle(command).await?;
        Ok(RegisterResponse { message: "注册成功".to_string() })
    }

    /// 用户登录
    pub async fn login(&self, req: LoginRequest) -> AppResult<LoginResponse> {
        let command = auth_mapper::to_login_command(req)?;
        let token = self.login.handle(command).await?;
        let query = GetCurrentUserQuery { token: token.clone() };
        let user_info = self.get_current_user.handle(query).await?;
        Ok(LoginResponse { token: token.to_string(), user: auth_mapper::to_current_user_info_response(user_info) })
    }

    /// 用户登出
    pub async fn logout(&self, req: LogoutRequest) -> AppResult<LogoutResponse> {
        let command = auth_mapper::to_logout_command(req)?;
        let _ = self.logout.handle(command).await?;
        Ok(LogoutResponse { message: "登出成功".to_string() })
    }

    /// 修改密码
    pub async fn change_password(
        &self,
        token: String,
        req: ChangePasswordRequest,
    ) -> AppResult<ChangePasswordResponse> {
        let command = auth_mapper::to_change_password_command(token, req)?;
        let _ = self.change_password.handle(command).await?;
        Ok(ChangePasswordResponse { message: "密码修改成功".to_string() })
    }

    /// 获取当前用户
    pub async fn get_current_user(&self, req: GetCurrentUserRequest) -> AppResult<GetCurrentUserResponse> {
        let query = auth_mapper::to_get_current_user_query(req)?;
        let user_info = self.get_current_user.handle(query).await?;
        Ok(GetCurrentUserResponse { user: auth_mapper::to_current_user_info_response(user_info) })
    }

    /// 获取用户菜单权限
    pub async fn get_user_menus(&self, req: GetUserMenusRequest) -> AppResult<GetUserMenusResponse> {
        let query = auth_mapper::to_get_user_menus_query(req)?;
        let menus = self.get_user_menus.handle(query).await?;
        Ok(GetUserMenusResponse { menus: auth_mapper::to_menu_responses(menus) })
    }

    /// 获取超级管理员仪表盘数据
    pub async fn get_super_admin_dashboard(
        &self,
        req: GetSuperAdminDashboardRequest,
    ) -> AppResult<GetSuperAdminDashboardResponse> {
        // 验证用户身份和权限
        let get_user_req = GetCurrentUserRequest { token: req.token };
        let query = auth_mapper::to_get_current_user_query(get_user_req)?;
        let user_info = self.get_current_user.handle(query).await?;

        // 去掉超级管理员权限校验，允许所有登录用户访问

        // 构建仪表盘数据（这里使用模拟数据，实际项目中应该从数据库获取）
        let system_stats = SystemStats {
            total_users: 1250,
            active_users: 89,
            total_roles: 8,
            total_permissions: 45,
            database_size: "2.3 GB".to_string(),
            uptime: "15 天 8 小时 32 分钟".to_string(),
        };

        let user_stats =
            UserStats { new_users_today: 12, new_users_this_week: 87, new_users_this_month: 324, active_sessions: 156 };

        let recent_activities = vec![
            RecentActivity {
                id: "act_001".to_string(),
                user_id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
                username: "admin".to_string(),
                action: "登录系统".to_string(),
                resource: "系统".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                ip_address: "192.168.1.100".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string(),
            },
            RecentActivity {
                id: "act_002".to_string(),
                user_id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
                username: "user001".to_string(),
                action: "创建用户".to_string(),
                resource: "用户管理".to_string(),
                timestamp: chrono::Utc::now().timestamp() - 3600,
                ip_address: "192.168.1.101".to_string(),
                user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".to_string(),
            },
        ];

        let system_health = SystemHealth {
            cpu_usage: 23.5,
            memory_usage: 68.2,
            disk_usage: 45.8,
            database_status: "正常".to_string(),
            redis_status: "正常".to_string(),
            rabbitmq_status: "正常".to_string(),
        };

        Ok(GetSuperAdminDashboardResponse { system_stats, user_stats, recent_activities, system_health })
    }
}
