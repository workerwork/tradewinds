use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::permission::PermissionId;

/// 根据权限ID查询权限查询
///
/// 参数：
/// - permission_id: 权限ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionByIdQuery {
    pub permission_id: PermissionId,
}
