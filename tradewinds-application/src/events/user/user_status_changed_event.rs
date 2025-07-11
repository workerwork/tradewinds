use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusChangedEvent {
    pub user_id: String,
    pub username: String,
    pub status: String,
    pub changed_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserStatusChangedEvent {
    pub fn new(user_id: &str, username: &str, status: &str, changed_by: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            status: status.to_string(),
            changed_by: changed_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserStatusChangedEvent {
    fn event_type(&self) -> &'static str {
        "user.status_changed"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
