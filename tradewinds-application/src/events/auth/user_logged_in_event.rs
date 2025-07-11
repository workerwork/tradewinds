use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedInEvent {
    pub user_id: String,
    pub username: String,
    pub ip_address: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl UserLoggedInEvent {
    pub fn new(user_id: &str, username: &str, ip_address: Option<&str>) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            ip_address: ip_address.map(|s| s.to_string()),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserLoggedInEvent {
    fn event_type(&self) -> &'static str {
        "user.logged_in"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
