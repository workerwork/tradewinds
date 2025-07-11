use crate::entities::User;
use crate::value_objects::user::UserId;
use async_trait::async_trait;
use tradewinds_error::AppResult;

// 用户服务接口, 定义了用户服务的基本操作
// 查询不到用户时, 返回 AppError::NotFound
#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<()>;
    async fn update(&self, user: &User) -> AppResult<()>;
    async fn delete(&self, id: &UserId) -> AppResult<()>;
    async fn get_by_id(&self, id: &UserId) -> AppResult<User>;
    async fn get_by_ids(&self, ids: &[UserId]) -> AppResult<Vec<User>>;
    async fn get_by_username(&self, username: &str) -> AppResult<User>;
    async fn get_by_email(&self, email: &str) -> AppResult<User>;
    async fn list(&self, page: u32, page_size: u32) -> AppResult<Vec<User>>;
    async fn exists_by_username(&self, username: &str) -> AppResult<bool>;
    async fn exists_by_email(&self, email: &str) -> AppResult<bool>;
    async fn assign_role(&self, user_id: &str, role_id: &str) -> AppResult<()>;
    async fn revoke_role(&self, user_id: &str, role_id: &str) -> AppResult<()>;
}
