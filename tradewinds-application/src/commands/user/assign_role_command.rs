use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    role::RoleId, 
    user::UserId,
};

/// 分配角色给用户命令
///
/// 参数：
/// - user_id: 用户ID
/// - role_id: 角色ID
/// - assigned_by: 分配者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleCommand {
    pub user_id: UserId,
    pub role_id: RoleId,
    pub assigned_by: Option<UserId>,
}
