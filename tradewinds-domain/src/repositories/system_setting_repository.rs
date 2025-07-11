use crate::entities::system_setting::SystemSetting;
use crate::value_objects::system_setting::{SystemSettingKey, SystemSettingValue};
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait SystemSettingRepository: Send + Sync {
    async fn get_by_key(&self, key: &SystemSettingKey) -> AppResult<Option<SystemSetting>>;
    async fn set_value(&self, key: &SystemSettingKey, value: &SystemSettingValue) -> AppResult<()>;
}
