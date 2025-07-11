use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::user::UserId;

/// 删除用户命令
///
/// 参数：
/// - id: 用户ID
/// - deleted_by: 删除者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserCommand {
    pub id: UserId,
    pub deleted_by: Option<UserId>,
}
