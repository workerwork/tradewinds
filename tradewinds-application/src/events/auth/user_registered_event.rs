use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserRegisteredEvent {
    pub fn new(user_id: &str, username: &str, email: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserRegisteredEvent {
    fn event_type(&self) -> &'static str {
        "user.registered"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
