use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::permission::PermissionName;

/// 根据权限名称查询权限查询
///
/// 参数：
/// - permission_name: 权限名称
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionByNameQuery {
    pub permission_name: PermissionName,
}
