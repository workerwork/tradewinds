use serde::{Deserialize, Serialize};

use tradewinds_domain::value_objects::{
    auth::{auth_password::Password, auth_username::AuthUsername},
    user::{user_email::Email, user_phone::Phone, user_real_name::RealName},
};

/// 注册命令
/// 用于注册新用户
///
/// 参数：
/// - username: 用户名，必须为6-20个字符，只能包含字母、数字和下划线
/// - email: 邮箱，必须为有效的邮箱地址
/// - password: 密码，必须为6-20个字符，必须包含至少一个字母和一个数字，不能包含空格
/// - real_name: 真实姓名，可选
/// - phone: 手机号，可选
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCommand {
    pub username: AuthUsername,
    pub email: Email,
    pub password: Password,
    pub real_name: Option<RealName>,
    pub phone: Option<Phone>,
}
