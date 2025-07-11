use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    permission::PermissionId,
    role::RoleId,
    user::UserId,
};

/// 撤销角色权限命令
///
/// 参数：
/// - role_id: 角色ID
/// - permission_id: 权限ID
/// - revoked_by: 撤销者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionCommand {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
    pub revoked_by: Option<UserId>,
}
