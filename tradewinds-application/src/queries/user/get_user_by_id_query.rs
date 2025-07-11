use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::user::UserId;

/// 根据ID获取用户查询
///
/// 参数：
/// - user_id: 用户ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByIdQuery {
    pub user_id: UserId,
}
