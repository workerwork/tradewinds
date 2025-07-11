#[rustfmt::skip]
use crate::{
    commands::*,
    queries::*,
};
use crate::queries::auth::user_info::CurrentUserInfo;
use tradewinds_domain::{
    entities::{permission::Permission, role::Role, user::User},
    value_objects::auth::{auth_token::Token, auth_username::AuthUsername},
};
use tradewinds_error::AppResult;

/// 认证服务接口
///
/// 定义了认证服务的基本操作，包括用户注册、登录、修改密码、登出和获取当前用户。
/// 这些操作通过命令和查询来实现。
///
/// 实现此接口的类型必须实现以下方法：
/// - `register`: 注册新用户
/// - `login`: 登录用户
/// - `change_password`: 修改密码
/// - `logout`: 登出用户
/// - `get_current_user`: 获取当前用户
#[async_trait::async_trait]
pub trait IAuthService: Send + Sync {
    async fn register(&self, cmd: RegisterCommand) -> AppResult<()>;
    async fn login(&self, cmd: LoginCommand) -> AppResult<Token>;
    async fn change_password(&self, cmd: ChangePasswordCommand) -> AppResult<()>;
    async fn logout(&self, cmd: LogoutCommand) -> AppResult<()>;
    async fn get_current_user(&self, query: GetCurrentUserQuery) -> AppResult<CurrentUserInfo>;
}
