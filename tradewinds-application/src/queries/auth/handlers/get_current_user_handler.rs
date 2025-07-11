#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::auth_service::IAuthService  ,
    queries::auth::{get_current_user_query::GetCurrentUserQuery, user_info::CurrentUserInfo},
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 获取当前用户查询处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 获取当前用户查询处理器
pub struct GetCurrentUserHandler {
    auth_service: Arc<dyn IAuthService>,
}

impl GetCurrentUserHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetCurrentUserQuery, CurrentUserInfo> for GetCurrentUserHandler {
    async fn handle(&self, query: GetCurrentUserQuery) -> AppResult<CurrentUserInfo> {
        self.auth_service.get_current_user(query).await
    }
}
