use derive_more::{Deref, Display, From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use tradewinds_error::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Display, From, Into, Deref, Serialize, Deserialize)]
pub struct UserRoleId(String);

impl UserRoleId {
    pub fn new(value: String) -> AppResult<Self> {
        if value.is_empty() {
            return Err(AppError::Validation("UserRole id is required".into()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}
