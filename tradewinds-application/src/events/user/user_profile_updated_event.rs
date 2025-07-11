use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdatedEvent {
    pub user_id: String,
    pub username: String,
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl UserProfileUpdatedEvent {
    pub fn new(user_id: &str, username: &str, real_name: Option<&str>, phone: Option<&str>) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            real_name: real_name.map(|s| s.to_string()),
            phone: phone.map(|s| s.to_string()),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserProfileUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "user.profile_updated"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
