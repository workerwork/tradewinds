use std::sync::Arc;

use tradewinds_error::AppResult;

use crate::{CommandHandler, commands::auth::logout_command::LogoutCommand, interfaces::auth_service::IAuthService};

/// 登出命令处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 登出命令处理器
pub struct LogoutHandler {
    auth_service: Arc<dyn IAuthService>,
}

impl LogoutHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<LogoutCommand, ()> for LogoutHandler {
    async fn handle(&self, command: LogoutCommand) -> AppResult<()> {
        self.auth_service.logout(command).await
    }
}
