#[rustfmt::skip]
use crate::{
    CommandHandler,
    commands::system_setting::set_system_setting_command::SetSystemSettingCommand,
    interfaces::system_setting_service::ISystemSettingService,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 设置系统设置命令处理器
///
/// 参数：
/// - system_setting_service: 系统设置服务
///
/// 返回：
/// - 设置系统设置命令处理器
pub struct SetSystemSettingHandler {
    pub system_setting_service: Arc<dyn ISystemSettingService>,
}

impl SetSystemSettingHandler {
    pub fn new(system_setting_service: Arc<dyn ISystemSettingService>) -> Self {
        Self { system_setting_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<SetSystemSettingCommand, ()> for SetSystemSettingHandler {
    async fn handle(&self, command: SetSystemSettingCommand) -> AppResult<()> {
        self.system_setting_service.set_value(command).await
    }
}
