use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::auth::{AuthUsername, Password};

/// 登录命令
/// 用于验证用户名和密码
///
/// 参数：
/// - username: 用户名，必须为6-20个字符，只能包含字母、数字和下划线
/// - password: 密码，必须为6-20个字符，必须包含至少一个字母和一个数字，不能包含空格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCommand {
    pub username: AuthUsername,
    pub password: Password,
}
