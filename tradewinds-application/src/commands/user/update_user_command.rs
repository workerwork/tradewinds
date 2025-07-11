use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::user::{
    user_avatar::Avatar, 
    user_id::UserId, 
    user_phone::Phone, 
    user_real_name::RealName,
    user_status::UserStatus,
    user_email::Email,
};
use tradewinds_domain::value_objects::role::RoleId;

/// 更新用户命令
///
/// 参数：
/// - id: 用户ID
/// - real_name: 真实姓名
/// - phone: 手机号
/// - avatar: 头像
/// - status: 状态
/// - email: 邮箱
/// - role_ids: 角色ID列表
/// - updated_by: 更新者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub id: UserId,
    pub real_name: Option<RealName>,
    pub phone: Option<Phone>,
    pub avatar: Option<Avatar>,
    pub status: Option<UserStatus>,
    pub email: Option<Email>,
    pub role_ids: Option<Vec<RoleId>>,
    pub updated_by: Option<UserId>,
}
