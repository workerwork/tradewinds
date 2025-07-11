use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCreatedEvent {
    pub role_id: String,
    pub name: String,
    pub created_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl RoleCreatedEvent {
    pub fn new(role_id: &str, name: &str, created_by: &str) -> Self {
        Self {
            role_id: role_id.to_string(),
            name: name.to_string(),
            created_by: created_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for RoleCreatedEvent {
    fn event_type(&self) -> &'static str {
        "role.created"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
