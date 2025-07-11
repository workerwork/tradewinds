use async_trait::async_trait;

use crate::entities::user::User;
use crate::value_objects::{
    auth::auth_username::AuthUsername,
    user::{user_email::Email, user_id::UserId, user_status::UserStatus},
};
use tradewinds_error::AppResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &AuthUsername) -> AppResult<Option<User>>;
    async fn find_by_ids(&self, ids: &[UserId]) -> AppResult<Vec<User>>;

    async fn exists_by_username(&self, username: &AuthUsername) -> AppResult<bool>;
    async fn exists_by_email(&self, email: &Email) -> AppResult<bool>;

    async fn count(&self) -> AppResult<u64>;

    async fn search(
        &self,
        username: Option<&AuthUsername>,
        phone: Option<&str>,
        email: Option<&Email>,
        status: Option<UserStatus>,
        show_deleted: Option<bool>,
        limit: u64,
        offset: u64,
    ) -> AppResult<(Vec<User>, u64)>;
}
