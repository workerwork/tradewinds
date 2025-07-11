use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UserStatus {
    #[default]
    Active,
    Inactive,
    Deleted,
}

impl UserStatus {
    pub fn from_i32(value: i32) -> AppResult<Self> {
        match value {
            0 => Ok(UserStatus::Active),
            1 => Ok(UserStatus::Inactive),
            2 => Ok(UserStatus::Deleted),
            _ => Err(AppError::Validation("User status can only be 0, 1, 2".to_string())),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            UserStatus::Active => 0,
            UserStatus::Inactive => 1,
            UserStatus::Deleted => 2,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, UserStatus::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, UserStatus::Inactive)
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self, UserStatus::Deleted)
    }

    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl FromStr for UserStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "deleted" => Ok(UserStatus::Deleted),
            _ => Err(AppError::Validation(format!("Invalid user status: {}", s))),
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}
