use derive_more::{Deref, Display, From, Into};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tradewinds_error::{AppError, AppResult};
use uuid::Uuid;

/// 权限ID
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize, Default, Display, From, Into, Deref,
)]
pub struct PermissionId(String);

impl PermissionId {
    pub fn new(value: String) -> AppResult<Self> {
        if value.is_empty() {
            return Err(AppError::Validation("Permission id is required".into()));
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

// impl Display for PermissionId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl FromStr for PermissionId {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(AppError::Validation("Permission ID cannot be empty".into()));
        }
        Ok(Self(s.to_string()))
    }
}

// impl From<String> for PermissionId {
//     fn from(value: String) -> Self {
//         Self(value)
//     }
// }

impl AsRef<String> for PermissionId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
