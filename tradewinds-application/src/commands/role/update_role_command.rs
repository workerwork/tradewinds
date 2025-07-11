use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    role::{
        RoleDescription,
        RoleId,
        RoleName,
        RoleStatus,
    },
    user::UserId,
    permission::PermissionId,
};

/// 更新角色命令
///
/// 参数：
/// - id: 角色ID
/// - name: 角色名称
/// - description: 角色描述
/// - status: 角色状态
/// - updated_by: 更新者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleCommand {
    pub id: RoleId,
    pub name: Option<RoleName>,
    pub description: Option<RoleDescription>,
    pub status: Option<RoleStatus>,
    pub updated_by: Option<UserId>,
    pub permissions: Option<Vec<PermissionId>>,
}
