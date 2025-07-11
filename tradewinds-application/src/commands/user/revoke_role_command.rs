use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    role::RoleId,
    user::UserId,
};

/// 撤销用户角色命令
///
/// 参数：
/// - user_id: 用户ID
/// - role_id: 角色ID
/// - revoked_by: 撤销者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRoleCommand {
    pub user_id: UserId,
    pub role_id: RoleId,
    pub revoked_by: Option<UserId>,
}
