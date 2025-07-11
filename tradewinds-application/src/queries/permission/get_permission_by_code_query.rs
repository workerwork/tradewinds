use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::permission::PermissionCode;

/// 根据权限代码查询权限查询
///
/// 参数：
/// - permission_code: 权限代码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionByCodeQuery {
    pub permission_code: PermissionCode,
}
