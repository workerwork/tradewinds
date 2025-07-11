use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleRevokedEvent {
    pub user_id: String,
    pub username: String,
    pub role_id: String,
    pub role_name: String,
    pub revoked_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserRoleRevokedEvent {
    pub fn new(user_id: &str, username: &str, role_id: &str, role_name: &str, revoked_by: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            role_id: role_id.to_string(),
            role_name: role_name.to_string(),
            revoked_by: revoked_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserRoleRevokedEvent {
    fn event_type(&self) -> &'static str {
        "user.role_revoked"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
