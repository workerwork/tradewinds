use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    permission::PermissionId,
    role::RoleId,
    user::UserId,
};

/// 分配权限给角色命令
///
/// 参数：
/// - role_id: 角色ID
/// - permission_ids: 权限ID列表
/// - assigned_by: 分配者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignPermissionCommand {
    pub role_id: RoleId,
    pub permission_ids: Vec<PermissionId>,
    pub assigned_by: UserId, // 设为必填，方便审计
}
