use crate::value_objects::{RoleId, UserId};
use async_trait::async_trait;
use tradewinds_error::AppResult;

#[async_trait]
pub trait UserRoleService: Send + Sync {
    /// 给用户分配角色
    async fn assign_role(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<()>;

    /// 给用户撤销角色
    async fn revoke_role(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<()>;

    /// 查询用户拥有的所有角色ID
    async fn list_roles_by_user(&self, user_id: &UserId) -> AppResult<Vec<RoleId>>;

    /// 判断用户是否拥有指定角色
    async fn has_role(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<bool>;
}
