use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    role::RoleId,
    user::UserId,
};

/// 删除角色命令
///
/// 参数：
/// - id: 角色ID
/// - deleted_by: 删除者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleCommand {
    pub id: RoleId,
    pub deleted_by: Option<UserId>,
}
