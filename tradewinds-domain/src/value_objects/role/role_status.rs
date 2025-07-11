use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RoleStatus {
    #[default]
    Active,
    Inactive,
    Deleted,
}

impl RoleStatus {
    pub fn from_i32(value: i32) -> AppResult<Self> {
        match value {
            0 => Ok(RoleStatus::Active),
            1 => Ok(RoleStatus::Inactive),
            2 => Ok(RoleStatus::Deleted),
            _ => Err(AppError::Validation("Role status can only be 0, 1, 2".to_string())),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            RoleStatus::Active => 0,
            RoleStatus::Inactive => 1,
            RoleStatus::Deleted => 2,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, RoleStatus::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, RoleStatus::Inactive)
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self, RoleStatus::Deleted)
    }

    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl FromStr for RoleStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(RoleStatus::Active),
            "inactive" => Ok(RoleStatus::Inactive),
            "deleted" => Ok(RoleStatus::Deleted),
            _ => Err(AppError::Validation(format!("Invalid role status: {}", s))),
        }
    }
}

impl fmt::Display for RoleStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}
