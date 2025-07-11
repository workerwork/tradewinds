use std::{fmt, str::FromStr};

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct Avatar(String);

impl Avatar {
    pub fn new<S: Into<String>>(value: S) -> AppResult<Self> {
        let value = value.into();
        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err(AppError::Validation("Avatar URL must start with http:// or https://".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for Avatar {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
