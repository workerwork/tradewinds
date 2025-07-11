use crate::value_objects::system_setting::{SystemSettingId, SystemSettingKey, SystemSettingValue};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct SystemSetting {
    pub id: SystemSettingId,
    pub key: SystemSettingKey,
    pub value: SystemSettingValue,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}
