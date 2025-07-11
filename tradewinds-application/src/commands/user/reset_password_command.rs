use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    user::UserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordCommand {
    pub id: UserId,
    pub reset_by: Option<UserId>,
}
