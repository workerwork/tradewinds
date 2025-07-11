use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::auth::Token;

/// 登出命令
/// 用于使令牌失效
///
/// 参数：
/// - token: 令牌
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutCommand {
    pub token: Token,
}
