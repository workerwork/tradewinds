use async_trait::async_trait;

use crate::entities::{user::User, user_role::UserRole};
use crate::value_objects::role::RoleId;
use crate::value_objects::user::UserId;
use tradewinds_error::AppResult;

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn create(&self, user_role: &UserRole) -> AppResult<()>;
    async fn delete(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<()>;
    async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<UserRole>>;
    async fn find_users_by_role_id(&self, role_id: &RoleId) -> AppResult<Vec<User>>;
    async fn exists(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<bool>;
}
