use crate::api::dtos::system_setting_dto::{GetSystemSettingRequest, SetSystemSettingRequest};
use tradewinds_application::commands::system_setting::set_system_setting_command::SetSystemSettingCommand;
use tradewinds_application::queries::system_setting::get_system_setting_query::GetSystemSettingQuery;
use tradewinds_domain::value_objects::system_setting::{SystemSettingKey, SystemSettingValue};

pub fn to_get_system_setting_query(req: GetSystemSettingRequest) -> GetSystemSettingQuery {
    GetSystemSettingQuery { key: SystemSettingKey::new(req.key).unwrap() }
}

pub fn to_set_system_setting_command(req: SetSystemSettingRequest) -> SetSystemSettingCommand {
    SetSystemSettingCommand {
        key: SystemSettingKey::new(req.key).unwrap(),
        value: SystemSettingValue::new(req.value).unwrap(),
    }
}
