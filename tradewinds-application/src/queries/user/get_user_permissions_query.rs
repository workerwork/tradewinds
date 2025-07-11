use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::user::UserId;

/// 获取用户权限查询
///
/// 参数：
/// - user_id: 用户ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPermissionsQuery {
    pub user_id: UserId,
}
