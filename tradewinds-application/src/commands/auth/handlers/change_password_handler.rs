use std::sync::Arc;

use tradewinds_error::AppResult;

use crate::{
    CommandHandler, commands::auth::change_password_command::ChangePasswordCommand,
    interfaces::auth_service::IAuthService,
};

/// 修改密码命令处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 修改密码命令处理器
pub struct ChangePasswordHandler {
    auth_service: Arc<dyn IAuthService>,
}

impl ChangePasswordHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<ChangePasswordCommand, ()> for ChangePasswordHandler {
    async fn handle(&self, command: ChangePasswordCommand) -> AppResult<()> {
        self.auth_service.change_password(command).await
    }
}
