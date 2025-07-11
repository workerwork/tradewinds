use std::sync::Arc;

use crate::api::{
    dtos::system_setting_dto::{
        GetSystemSettingRequest, GetSystemSettingResponse, SetSystemSettingRequest, SetSystemSettingResponse,
    },
    mappers::system_setting_mapper,
};
use tradewinds_application::CommandHandler;
use tradewinds_application::QueryHandler;
use tradewinds_application::commands::system_setting::handlers::set_system_setting_handler::SetSystemSettingHandler;
use tradewinds_application::commands::system_setting::set_system_setting_command::SetSystemSettingCommand;
use tradewinds_application::interfaces::system_setting_service::ISystemSettingService;
use tradewinds_application::queries::system_setting::get_system_setting_query::GetSystemSettingQuery;
use tradewinds_application::queries::system_setting::handlers::get_system_setting_handler::GetSystemSettingHandler;
use tradewinds_domain::entities::system_setting::SystemSetting;
use tradewinds_error::AppResult;

#[derive(Clone)]
pub struct SystemSettingController {
    get_handler: Arc<dyn QueryHandler<GetSystemSettingQuery, SystemSetting>>,
    set_handler: Arc<dyn CommandHandler<SetSystemSettingCommand, ()>>,
}

impl SystemSettingController {
    pub fn new(
        get_handler: Arc<dyn QueryHandler<GetSystemSettingQuery, SystemSetting>>,
        set_handler: Arc<dyn CommandHandler<SetSystemSettingCommand, ()>>,
    ) -> Self {
        Self { get_handler, set_handler }
    }

    pub async fn get_by_key(&self, req: GetSystemSettingRequest) -> AppResult<GetSystemSettingResponse> {
        let query = system_setting_mapper::to_get_system_setting_query(req);
        let setting = self.get_handler.handle(query).await?;
        Ok(GetSystemSettingResponse {
            key: setting.key.value().to_string(),
            value: setting.value.value().to_string(),
            description: setting.description,
        })
    }

    pub async fn set_value(&self, req: SetSystemSettingRequest) -> AppResult<SetSystemSettingResponse> {
        let cmd = system_setting_mapper::to_set_system_setting_command(req);
        self.set_handler.handle(cmd).await?;
        Ok(SetSystemSettingResponse { success: true })
    }

    pub fn assemble(system_setting_service: Arc<dyn ISystemSettingService>) -> Self {
        Self::new(
            Arc::new(GetSystemSettingHandler::new(system_setting_service.clone())),
            Arc::new(SetSystemSettingHandler::new(system_setting_service.clone())),
        )
    }
}
