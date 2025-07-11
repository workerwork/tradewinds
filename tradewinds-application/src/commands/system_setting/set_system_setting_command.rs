use serde::{Deserialize, Serialize};
use tradewinds_domain::value_objects::system_setting::{SystemSettingKey, SystemSettingValue};

/// 设置系统设置命令
///
/// 参数：
/// - key: 系统设置键
/// - value: 系统设置值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSystemSettingCommand {
    pub key: SystemSettingKey,
    pub value: SystemSettingValue,
}
