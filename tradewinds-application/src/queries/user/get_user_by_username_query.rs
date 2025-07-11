use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::auth::auth_username::AuthUsername;

/// 根据用户名获取用户查询
///
/// 参数：
/// - username: 用户名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByUsernameQuery {
    pub username: AuthUsername,
}
