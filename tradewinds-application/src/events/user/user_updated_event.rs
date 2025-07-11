use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: String,
    pub username: String,
    pub updated_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserUpdatedEvent {
    pub fn new(user_id: &str, username: &str, updated_by: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            updated_by: updated_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "user.updated"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
