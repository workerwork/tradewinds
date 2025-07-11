use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::permission::PermissionType;

/// 根据权限类型查询权限列表查询
///
/// 参数：
/// - permission_type: 权限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsByTypeQuery {
    pub permission_type: PermissionType,
}
