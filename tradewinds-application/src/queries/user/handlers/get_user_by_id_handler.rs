#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::user_service::IUserService,
    queries::user::get_user_by_id_query::GetUserByIdQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::user::User;
use tradewinds_domain::value_objects::user::UserId;
use tradewinds_error::AppResult;

/// 根据ID获取用户查询处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 根据ID获取用户查询处理器
pub struct GetUserByIdHandler {
    user_service: Arc<dyn IUserService>,
}

impl GetUserByIdHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetUserByIdQuery, User> for GetUserByIdHandler {
    async fn handle(&self, query: GetUserByIdQuery) -> AppResult<User> {
        self.user_service.get_user_by_id(query).await
    }
}
