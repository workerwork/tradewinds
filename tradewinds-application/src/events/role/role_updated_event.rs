use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUpdatedEvent {
    pub role_id: String,
    pub name: String,
    pub updated_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl RoleUpdatedEvent {
    pub fn new(role_id: &str, name: &str, updated_by: &str) -> Self {
        Self {
            role_id: role_id.to_string(),
            name: name.to_string(),
            updated_by: updated_by.to_string(),
            occurred_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Event for RoleUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "role.updated"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
