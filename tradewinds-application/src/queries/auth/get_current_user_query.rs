use tradewinds_domain::value_objects::auth::Token;

/// 获取当前用户查询
///
/// 参数：
/// - token: 令牌
#[derive(Debug, Clone)]
pub struct GetCurrentUserQuery {
    pub token: Token,
}
