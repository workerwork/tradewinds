use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Deref)]
pub struct PermissionSort(i32);

impl PermissionSort {
    pub fn new(value: i32) -> AppResult<Self> {
        if value < 0 {
            return Err(AppError::Validation("Sort must be non-negative".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl FromStr for PermissionSort {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<i32>()?;
        Self::new(value)
    }
}

impl fmt::Display for PermissionSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
