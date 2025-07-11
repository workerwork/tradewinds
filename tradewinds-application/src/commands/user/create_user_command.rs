use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    auth::{
        auth_password::Password, 
        auth_username::AuthUsername
    },
    user::{
        user_avatar::Avatar, 
        user_email::Email, 
        user_phone::Phone, 
        user_real_name::RealName,
        user_id::UserId,
    },
    role::RoleId,
};

/// 创建用户命令
///
/// 参数：
/// - username: 用户名
/// - email: 邮箱
/// - password: 密码
/// - real_name: 真实姓名
/// - phone: 手机号
/// - avatar: 头像
/// - role_ids: 角色ID列表
/// - created_by: 创建者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub username: AuthUsername,
    pub email: Email,
    pub password: Password,
    pub real_name: Option<RealName>,
    pub phone: Option<Phone>,
    pub avatar: Option<Avatar>,
    pub role_ids: Option<Vec<RoleId>>,
    pub created_by: Option<UserId>,
}
