use crate::entities::user::User;
use crate::value_objects::{
    auth::{auth_password::Password, auth_username::AuthUsername},
    role::RoleId,
    user::{
        user_avatar::Avatar, user_email::Email, user_phone::Phone, user_real_name::RealName, user_status::UserStatus,
    },
};
use chrono::Utc;
use tradewinds_error::{AppError, AppResult};

#[derive(Debug)]
pub struct UserAggregate {
    pub user: User,
    pub roles: Vec<RoleId>,
}

impl UserAggregate {
    /// 创建新用户（注册逻辑）
    pub fn create(
        username: AuthUsername,
        email: Email,
        password: Password,
        real_name: Option<RealName>,
        phone: Option<Phone>,
        avatar: Option<Avatar>,
    ) -> AppResult<Self> {
        // 调用 User 的 create 方法创建用户
        let user = User::create(username, email, password, real_name, phone, avatar);
        Ok(Self { user, roles: Vec::new() })
    }

    /// 创建新用户并分配角色
    pub fn create_with_roles(
        username: AuthUsername,
        email: Email,
        password: Password,
        real_name: Option<RealName>,
        phone: Option<Phone>,
        avatar: Option<Avatar>,
        role_ids: Vec<RoleId>,
    ) -> AppResult<Self> {
        // 调用 User 的 create 方法创建用户
        let user = User::create(username, email, password, real_name, phone, avatar);
        Ok(Self { user, roles: role_ids })
    }

    /// 更新用户资料
    pub fn update(
        &mut self,
        real_name: Option<RealName>,
        phone: Option<Phone>,
        avatar: Option<Avatar>,
        status: Option<UserStatus>,
        email: Option<Email>,
        role_ids: Option<Vec<RoleId>>,
    ) -> AppResult<()> {
        let new_status = status.unwrap_or(self.user.status);
        self.user.update_profile(real_name, phone, avatar, new_status, email);
        if let Some(role_ids) = role_ids {
            self.roles = role_ids;
        }
        self.touch();
        Ok(())
    }

    /// 删除用户
    pub fn delete(&mut self) -> AppResult<()> {
        if self.user.status == UserStatus::Deleted {
            return Err(AppError::Validation("User already deleted".into()));
        }
        self.user.status = UserStatus::Deleted;
        self.touch();
        Ok(())
    }

    /// 恢复用户
    pub fn restore(&mut self) -> AppResult<()> {
        self.user.status = UserStatus::Active;
        self.touch();
        Ok(())
    }

    /// 分配角色（避免重复）
    pub fn assign_role(&mut self, role_id: &RoleId) -> AppResult<()> {
        if !self.roles.contains(&role_id) {
            self.roles.push(role_id.clone());
            self.touch();
        }
        Ok(())
    }

    /// 移除角色
    pub fn revoke_role(&mut self, role_id: &RoleId) -> AppResult<()> {
        self.roles.retain(|r| r != role_id);
        self.touch();
        Ok(())
    }

    /// 激活用户
    pub fn activate(&mut self) -> AppResult<()> {
        if self.user.is_active() {
            return Err(AppError::Validation("User already active".into()));
        }
        self.user.status = UserStatus::Active;
        self.touch();
        Ok(())
    }

    /// 重置用户密码
    pub fn reset_password(&mut self, new_password: Password) {
        self.user.reset_password(new_password);
        self.touch();
    }

    /// 内部更新时间戳
    fn touch(&mut self) {
        self.user.updated_at = Utc::now().timestamp();
    }
}
