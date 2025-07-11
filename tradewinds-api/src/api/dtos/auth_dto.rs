use serde::{Deserialize, Serialize};

use tradewinds_application::queries::auth::menu_info::MenuInfo;

#[rustfmt::skip]
use crate::api::dtos::{
    user_dto::UserResponse,
    role_dto::RoleResponse, 
    permission_dto::PermissionResponse, 
};

// 登录请求
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 登录响应
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: CurrentUserInfoResponse,
}

// 登出请求
#[derive(Debug, Deserialize, Serialize)]
pub struct LogoutRequest {
    pub token: String,
}

// 登出响应
#[derive(Debug, Deserialize, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

// 注册请求
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
}

// 注册响应
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

// 修改密码请求
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

// 修改密码响应
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

// 获取当前用户请求
#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentUserRequest {
    pub token: String,
}

// 获取当前用户响应
#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentUserResponse {
    pub user: CurrentUserInfoResponse,
}

// 验证令牌请求
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

// 验证令牌响应
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyTokenResponse {
    pub token: String,
    pub user: CurrentUserInfoResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUserInfoResponse {
    pub user: UserResponse,
    pub roles: Vec<RoleResponse>,
    pub permissions: Vec<PermissionResponse>,
}

// 新增：获取用户菜单权限
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserMenusRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserMenusResponse {
    pub menus: Vec<MenuResponse>,
}

// 超级管理员仪表盘数据
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSuperAdminDashboardRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSuperAdminDashboardResponse {
    pub system_stats: SystemStats,
    pub user_stats: UserStats,
    pub recent_activities: Vec<RecentActivity>,
    pub system_health: SystemHealth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_users: u64,
    pub active_users: u64,
    pub total_roles: u64,
    pub total_permissions: u64,
    pub database_size: String,
    pub uptime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub new_users_today: u64,
    pub new_users_this_week: u64,
    pub new_users_this_month: u64,
    pub active_sessions: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentActivity {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub action: String,
    pub resource: String,
    pub timestamp: i64,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemHealth {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub database_status: String,
    pub redis_status: String,
    pub rabbitmq_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub children: Vec<MenuResponse>,
}

impl From<MenuInfo> for MenuResponse {
    fn from(info: MenuInfo) -> Self {
        MenuResponse {
            id: info.id,
            name: info.name,
            code: info.code,
            path: info.path,
            component: info.component,
            icon: info.icon,
            sort: info.sort,
            parent_id: info.parent_id,
            children: info.children.into_iter().map(MenuResponse::from).collect(),
        }
    }
}
