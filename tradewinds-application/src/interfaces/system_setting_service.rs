use crate::{
    queries::system_setting::get_system_setting_query::GetSystemSettingQuery,
    commands::system_setting::set_system_setting_command::SetSystemSettingCommand,
};
use tradewinds_domain::entities::system_setting::SystemSetting;
use tradewinds_error::AppResult;

#[async_trait::async_trait]
pub trait ISystemSettingService: Send + Sync {
    async fn get_by_key(&self, query: GetSystemSettingQuery) -> AppResult<Option<SystemSetting>>;
    async fn set_value(&self, cmd: SetSystemSettingCommand) -> AppResult<()>;
}
