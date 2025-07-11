use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PermissionStatus {
    #[default]
    Active,
    Inactive,
    Deleted,
}

impl PermissionStatus {
    pub fn from_i32(value: i32) -> AppResult<Self> {
        match value {
            0 => Ok(PermissionStatus::Active),
            1 => Ok(PermissionStatus::Inactive),
            2 => Ok(PermissionStatus::Deleted),
            _ => Err(AppError::Validation("Permission status can only be 0, 1, 2".to_string())),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            PermissionStatus::Active => 0,
            PermissionStatus::Inactive => 1,
            PermissionStatus::Deleted => 2,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, PermissionStatus::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, PermissionStatus::Inactive)
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self, PermissionStatus::Deleted)
    }

    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl FromStr for PermissionStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(PermissionStatus::Active),
            "inactive" => Ok(PermissionStatus::Inactive),
            "deleted" => Ok(PermissionStatus::Deleted),
            _ => Err(AppError::Validation(format!("Invalid permission status: {}", s))),
        }
    }
}

impl fmt::Display for PermissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}
