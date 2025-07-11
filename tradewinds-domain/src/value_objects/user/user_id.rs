use std::{fmt, str::FromStr};

use derive_more::Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Deref)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: String) -> AppResult<Self> {
        if value.is_empty() {
            return Err(AppError::Validation("User id is required".into()));
        }
        Ok(Self(value))
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for UserId {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(AppError::Validation("User ID cannot be empty".into()));
        }
        // Basic UUID validation could be added here if needed
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
