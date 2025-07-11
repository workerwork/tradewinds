use crate::value_objects::{role::role_id::RoleId, user::user_id::UserId, user_role::user_role_id::UserRoleId};
use chrono::Utc;

/// 用户-角色关联实体
/// 表示用户和角色之间的多对多关系
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRole {
    pub id: UserRoleId,
    pub user_id: UserId,
    pub role_id: RoleId,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserRole {
    pub fn new(user_id: UserId, role_id: RoleId) -> Self {
        let now = Utc::now().timestamp();
        Self { id: UserRoleId::new_v4(), user_id, role_id, created_at: now, updated_at: now }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn role_id(&self) -> &RoleId {
        &self.role_id
    }

    /// 为用户创建角色关联集合
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `role_ids` - 角色ID列表
    ///
    /// # 返回
    /// * `Vec<(String, String, String)>` - (id, user_id, role_id) 元组列表
    pub fn create_associations(user_id: UserId, role_ids: &[RoleId]) -> Vec<(String, String, String)> {
        let user_id_str = user_id.to_string();
        role_ids
            .iter()
            .map(|role_id| {
                let id = UserRoleId::new_v4();
                (id.to_string(), user_id_str.clone(), role_id.to_string())
            })
            .collect()
    }
}
