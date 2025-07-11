use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::role::RoleName;

/// 根据名称获取角色查询
///
/// 参数：
/// - role_name: 角色名称
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleByNameQuery {
    pub role_name: RoleName,
}
