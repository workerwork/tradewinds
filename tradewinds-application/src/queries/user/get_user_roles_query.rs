use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::user::UserId;

/// 获取用户角色查询
///
/// 参数：
/// - user_id: 用户ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRolesQuery {
    pub user_id: UserId,
}
