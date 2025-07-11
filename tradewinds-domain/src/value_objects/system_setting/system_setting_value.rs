use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SystemSettingValue(String);

impl SystemSettingValue {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("SystemSettingValue 不能为空".to_string());
        }
        Ok(Self(value))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for SystemSettingValue {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemSettingValue::new(s.to_string())
    }
}

impl fmt::Display for SystemSettingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
