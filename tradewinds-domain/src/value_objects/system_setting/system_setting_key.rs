use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SystemSettingKey(String);

impl SystemSettingKey {
    pub fn new(key: String) -> Result<Self, String> {
        if key.trim().is_empty() {
            return Err("SystemSettingKey 不能为空".to_string());
        }
        // 可加更多格式校验
        Ok(Self(key))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for SystemSettingKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemSettingKey::new(s.to_string())
    }
}

impl fmt::Display for SystemSettingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
