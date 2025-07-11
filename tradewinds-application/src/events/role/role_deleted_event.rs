use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tradewinds_domain::services::Event;
use tradewinds_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDeletedEvent {
    pub role_id: String,
    pub deleted_by: String,
    pub occurred_at: DateTime<Utc>,
}

impl RoleDeletedEvent {
    pub fn new(role_id: &str, deleted_by: &str) -> Self {
        Self { role_id: role_id.to_string(), deleted_by: deleted_by.to_string(), occurred_at: Utc::now() }
    }
}

#[async_trait]
impl Event for RoleDeletedEvent {
    fn event_type(&self) -> &'static str {
        "role.deleted"
    }

    fn to_json(&self) -> AppResult<String> {
        serde_json::to_string(self).map_err(|e| e.into())
    }
}
