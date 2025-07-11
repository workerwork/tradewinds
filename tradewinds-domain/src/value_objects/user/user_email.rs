use std::str::FromStr;

use derive_more::{Deref, Display, From, Into};
use serde::{Deserialize, Serialize};
use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, From, Into, Deref, Display, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> AppResult<Self> {
        if value.is_empty() {
            return Err(AppError::Validation("Email is required".into()));
        }
        // A simple email validation
        if !value.contains('@') {
            return Err(AppError::Validation("Invalid email format".into()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for Email {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
