use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::role::RoleId;

/// 根据ID获取角色查询
///
/// 参数：
/// - role_id: 角色ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleByIdQuery {
    pub role_id: RoleId,
}
