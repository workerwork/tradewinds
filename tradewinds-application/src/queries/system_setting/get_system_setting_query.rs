use serde::{Deserialize, Serialize};
use tradewinds_domain::value_objects::system_setting::SystemSettingKey;

/// 根据系统设置键获取系统设置查询
///
/// 参数：
/// - key: 系统设置键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemSettingQuery {
    pub key: SystemSettingKey,
}
