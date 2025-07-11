use std::sync::Arc;

use tradewinds_error::AppResult;

use crate::{
    CommandHandler, commands::auth::register_command::RegisterCommand, interfaces::auth_service::IAuthService,
};

/// 注册命令处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 注册命令处理器
pub struct RegisterHandler {
    auth_service: Arc<dyn IAuthService>,
}

impl RegisterHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<RegisterCommand, ()> for RegisterHandler {
    async fn handle(&self, command: RegisterCommand) -> AppResult<()> {
        self.auth_service.register(command).await
    }
}
