#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::user_service::IUserService,
    commands::user::reset_password_command::ResetPasswordCommand,
};
use crate::interfaces::system_setting_service::ISystemSettingService;
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 重置用户密码命令处理器
///
/// 参数：
/// - user_service: 用户服务接口
///
/// 返回：
/// - 重置用户密码命令处理器
pub struct ResetPasswordHandler {
    user_service: Arc<dyn IUserService>,
    system_setting_service: Arc<dyn ISystemSettingService>,
}

impl ResetPasswordHandler {
    pub fn new(user_service: Arc<dyn IUserService>, system_setting_service: Arc<dyn ISystemSettingService>) -> Self {
        Self { user_service, system_setting_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<ResetPasswordCommand, ()> for ResetPasswordHandler {
    async fn handle(&self, command: ResetPasswordCommand) -> AppResult<()> {
        self.user_service.reset_password(command).await
    }
}   
