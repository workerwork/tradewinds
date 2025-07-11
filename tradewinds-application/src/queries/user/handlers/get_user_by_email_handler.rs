#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::user_service::IUserService,
    queries::user::get_user_by_email_query::GetUserByEmailQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::user::User;
use tradewinds_domain::value_objects::user::UserId;
use tradewinds_error::AppResult;

/// 根据邮箱获取用户查询处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 根据邮箱获取用户查询处理器
pub struct GetUserByEmailHandler {
    user_service: Arc<dyn IUserService>,
}

impl GetUserByEmailHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetUserByEmailQuery, User> for GetUserByEmailHandler {
    async fn handle(&self, query: GetUserByEmailQuery) -> AppResult<User> {
        self.user_service.get_user_by_email(query).await
    }
}
