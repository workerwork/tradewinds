use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionName(String);

impl PermissionName {
    pub fn new<S: Into<String>>(value: S) -> AppResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AppError::Validation("Permission name cannot be empty".into()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PermissionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for PermissionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for PermissionName {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
