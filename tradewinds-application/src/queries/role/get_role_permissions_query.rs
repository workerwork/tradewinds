use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::role::RoleId;

/// 根据角色ID获取角色权限查询
///
/// 参数：
/// - role_id: 角色ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolePermissionsQuery {
    pub role_id: RoleId,
}
