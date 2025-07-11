#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::user_service::IUserService,
    queries::user::get_user_roles_query::GetUserRolesQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::role::Role;
use tradewinds_domain::value_objects::user::UserId;
use tradewinds_error::AppResult;

/// 获取用户角色查询处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 获取用户角色查询处理器
pub struct GetUserRolesHandler {
    user_service: Arc<dyn IUserService>,
}

impl GetUserRolesHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetUserRolesQuery, Vec<Role>> for GetUserRolesHandler {
    async fn handle(&self, query: GetUserRolesQuery) -> AppResult<Vec<Role>> {
        self.user_service.get_user_roles(query).await
    }
}
