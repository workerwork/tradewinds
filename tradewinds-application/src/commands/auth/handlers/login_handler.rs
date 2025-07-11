use std::sync::Arc;

use tradewinds_domain::value_objects::auth::Token;
use tradewinds_error::AppResult;

use crate::{CommandHandler, commands::auth::login_command::LoginCommand, interfaces::auth_service::IAuthService};

/// 登录命令处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 登录命令处理器
pub struct LoginHandler {
    auth_service: Arc<dyn IAuthService>, // 认证服务
}

impl LoginHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<LoginCommand, Token> for LoginHandler {
    async fn handle(&self, command: LoginCommand) -> AppResult<Token> {
        self.auth_service.login(command).await
    }
}
