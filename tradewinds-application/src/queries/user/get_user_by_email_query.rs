use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::user::user_email::Email;

/// 根据邮箱获取用户查询
///
/// 参数：
/// - user_email: 邮箱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByEmailQuery {
    pub user_email: Email,
}
