use crate::{
    commands::system_setting::set_system_setting_command::SetSystemSettingCommand,
    interfaces::system_setting_service::ISystemSettingService,
    queries::system_setting::get_system_setting_query::GetSystemSettingQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::system_setting::SystemSetting;
use tradewinds_domain::repositories::system_setting_repository::SystemSettingRepository;
use tradewinds_error::AppResult;

#[derive(Clone)]
pub struct SystemSettingService {
    system_setting_repo: Arc<dyn SystemSettingRepository>,
}

impl SystemSettingService {
    pub fn new(system_setting_repo: Arc<dyn SystemSettingRepository>) -> Self {
        Self { system_setting_repo }
    }
}

#[async_trait::async_trait]
impl ISystemSettingService for SystemSettingService {
    async fn get_by_key(&self, query: GetSystemSettingQuery) -> AppResult<Option<SystemSetting>> {
        self.system_setting_repo.get_by_key(&query.key).await
    }

    async fn set_value(&self, cmd: SetSystemSettingCommand) -> AppResult<()> {
        self.system_setting_repo.set_value(&cmd.key, &cmd.value).await
    }
}
