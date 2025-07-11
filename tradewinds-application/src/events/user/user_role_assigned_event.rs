use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::event_bus::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleAssignedEvent {
    pub user_id: String,
    pub username: String,
    pub role_id: String,
    pub role_name: String,
    pub assigned_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserRoleAssignedEvent {
    pub fn new(user_id: &str, username: &str, role_id: &str, role_name: &str, assigned_by: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            role_id: role_id.to_string(),
            role_name: role_name.to_string(),
            assigned_by: assigned_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for UserRoleAssignedEvent {
    fn event_type(&self) -> &'static str {
        "user.role_assigned"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
