use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleCode(String);

impl RoleCode {
    pub fn new(value: String) -> AppResult<Self> {
        if value.trim().is_empty() {
            return Err(AppError::Validation("Role code cannot be empty".to_string()));
        }
        Ok(RoleCode(value))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RoleCode {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RoleCode::new(s.to_string())
    }
}
