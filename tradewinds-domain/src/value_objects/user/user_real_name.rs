use std::{fmt, str::FromStr};

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct RealName(String);

impl RealName {
    pub fn new<S: Into<String>>(value: S) -> AppResult<Self> {
        let value = value.into();
        if value.len() < 2 || value.len() > 50 {
            return Err(AppError::Validation("Real name length must be between 2 and 50 characters".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for RealName {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for RealName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
