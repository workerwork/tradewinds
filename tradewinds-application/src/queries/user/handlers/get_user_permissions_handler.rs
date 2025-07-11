#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::user_service::IUserService,
    queries::user::get_user_permissions_query::GetUserPermissionsQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 获取用户权限查询处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 获取用户权限查询处理器
pub struct GetUserPermissionsHandler {
    user_service: Arc<dyn IUserService>,
}

impl GetUserPermissionsHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetUserPermissionsQuery, Vec<Permission>> for GetUserPermissionsHandler {
    async fn handle(&self, query: GetUserPermissionsQuery) -> AppResult<Vec<Permission>> {
        self.user_service.get_user_permissions(query).await
    }
}
