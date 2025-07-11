use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    permission::PermissionId,
    role::{RoleCode, RoleDescription, RoleName, RoleStatus},
    user::UserId,
};

/// 创建角色命令
///
/// 参数：
/// - name: 角色名称
/// - description: 角色描述
/// - permissions: 权限ID列表
#[derive(Debug, Clone)]
pub struct CreateRoleCommand {
    pub name: RoleName,
    pub code: RoleCode,
    pub description: Option<RoleDescription>,
    pub permissions: Vec<PermissionId>,
    pub status: Option<RoleStatus>, // 新增，支持指定角色状态
}
