use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::permission::PermissionId;

/// 根据父权限ID查询权限列表查询
///
/// 参数：
/// - parent_id: 父权限ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsByParentIdQuery {
    pub parent_id: PermissionId,
}
