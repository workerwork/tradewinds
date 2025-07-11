use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SystemSettingId(String);

impl SystemSettingId {
    pub fn new(id: String) -> Result<Self, String> {
        if id.trim().is_empty() {
            return Err("SystemSettingId 不能为空".to_string());
        }
        Ok(Self(id))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for SystemSettingId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemSettingId::new(s.to_string())
    }
}

impl fmt::Display for SystemSettingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
