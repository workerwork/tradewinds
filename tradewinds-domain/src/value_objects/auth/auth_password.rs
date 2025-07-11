// Test comment
use std::{fmt, str::FromStr};

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> AppResult<Self> {
        if value.is_empty() {
            return Err(AppError::Validation("Password is required".into()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn verify(&self, password: &Password) -> AppResult<bool> {
        Ok(bcrypt::verify(&**password, &self.0)?)
    }
}

impl FromStr for Password {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
