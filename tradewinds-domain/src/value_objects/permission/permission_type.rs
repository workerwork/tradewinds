use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PermissionType {
    #[default]
    Menu,
    Button,
    Api,
}

impl PermissionType {
    pub fn from_i32(value: i32) -> AppResult<Self> {
        match value {
            0 => Ok(PermissionType::Menu),
            1 => Ok(PermissionType::Button),
            2 => Ok(PermissionType::Api),
            _ => Err(AppError::Validation(format!("Invalid permission type: {}", value))),
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            PermissionType::Menu => 0,
            PermissionType::Button => 1,
            PermissionType::Api => 2,
        }
    }

    pub fn from_string(value: String) -> AppResult<Self> {
        match value.to_lowercase().as_str() {
            "menu" => Ok(PermissionType::Menu),
            "button" => Ok(PermissionType::Button),
            "api" => Ok(PermissionType::Api),
            _ => Err(AppError::Validation(format!("Invalid permission type: {}", value))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PermissionType::Menu => "menu".to_string(),
            PermissionType::Button => "button".to_string(),
            PermissionType::Api => "api".to_string(),
        }
    }
}

impl fmt::Display for PermissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PermissionType::Menu => "menu",
            PermissionType::Button => "button",
            PermissionType::Api => "api",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for PermissionType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "menu" => Ok(PermissionType::Menu),
            "button" => Ok(PermissionType::Button),
            "api" => Ok(PermissionType::Api),
            _ => Err(AppError::Validation(format!("Invalid permission type: {}", s))),
        }
    }
}
