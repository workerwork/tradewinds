use serde::{Deserialize, Serialize};
use tradewinds_domain::value_objects::auth::auth_token::Token;

/// 获取用户菜单权限查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserMenusQuery {
    pub token: Token,
} 