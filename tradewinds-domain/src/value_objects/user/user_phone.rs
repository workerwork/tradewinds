use std::{fmt, str::FromStr};

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct Phone(String);

impl Phone {
    pub fn new<S: Into<String>>(value: S) -> AppResult<Self> {
        let value = value.into();
        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::Validation("Phone number can only contain numbers".to_string()));
        }
        if value.len() != 11 {
            return Err(AppError::Validation("Phone number length must be 11 digits".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for Phone {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
