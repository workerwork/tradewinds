use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::auth::{Password, Token};

/// 修改密码命令
/// 用于修改用户密码
///
/// 参数：
/// - token: 令牌
/// - old_password: 旧密码
/// - new_password: 新密码
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordCommand {
    pub token: Token,
    pub old_password: Password,
    pub new_password: Password,
}
