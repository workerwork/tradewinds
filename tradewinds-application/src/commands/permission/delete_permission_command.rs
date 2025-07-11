use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    permission::PermissionId,
    user::UserId,
};

/// 删除权限命令
///
/// 参数：
/// - permission_id: 权限ID
/// - deleted_by: 删除者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionCommand {
    pub permission_id: PermissionId,
    pub deleted_by: Option<UserId>,
}
