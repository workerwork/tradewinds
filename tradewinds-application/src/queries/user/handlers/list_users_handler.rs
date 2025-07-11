#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::user_service::IUserService,
    queries::user::list_users_query::ListUsersQuery,
};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{role::Role, user::User};
use tradewinds_error::AppResult;

/// 查询用户列表查询处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 查询用户列表查询处理器
pub struct ListUsersHandler {
    user_service: Arc<dyn IUserService>,
}

impl ListUsersHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListUsersQuery, PaginatedResult<(User, Vec<Role>)>> for ListUsersHandler {
    async fn handle(&self, query: ListUsersQuery) -> AppResult<PaginatedResult<(User, Vec<Role>)>> {
        self.user_service.list_users(query).await
    }
}
