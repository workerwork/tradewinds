use chrono::Utc;

use crate::value_objects::{
    auth::{auth_password::Password, auth_username::AuthUsername},
    user::{
        user_avatar::Avatar, user_email::Email, user_id::UserId, user_phone::Phone, user_real_name::RealName,
        user_status::UserStatus,
    },
};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: AuthUsername,
    pub email: Email,
    pub password: Password,
    pub real_name: Option<RealName>,
    pub phone: Option<Phone>,
    pub avatar: Option<Avatar>,
    pub status: UserStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn create(
        username: AuthUsername,
        email: Email,
        password: Password,
        real_name: Option<RealName>,
        phone: Option<Phone>,
        avatar: Option<Avatar>,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: UserId::new_v4(),
            username,
            email,
            password,
            real_name,
            phone,
            avatar,
            status: UserStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_profile(
        &mut self,
        real_name: Option<RealName>,
        phone: Option<Phone>,
        avatar: Option<Avatar>,
        status: UserStatus,
        email: Option<Email>,
    ) {
        if let Some(real_name) = real_name {
            self.real_name = Some(real_name);
        }
        if let Some(phone) = phone {
            self.phone = Some(phone);
        }
        if let Some(avatar) = avatar {
            self.avatar = Some(avatar);
        }
        self.status = status;
        if let Some(email) = email {
            self.email = email;
        };
        self.updated_at = Utc::now().timestamp();
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }

    pub fn is_deleted(&self) -> bool {
        self.status.is_deleted()
    }

    pub fn reset_password(&mut self, new_password: Password) {
        self.password = new_password;
        self.updated_at = Utc::now().timestamp();
    }
}
