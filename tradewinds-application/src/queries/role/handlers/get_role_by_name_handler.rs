#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::role_service::IRoleService,
    queries::role::get_role_by_name_query::GetRoleByNameQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::role::Role;
use tradewinds_error::AppResult;

/// 根据名称获取角色查询处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 根据名称获取角色查询处理器
pub struct GetRoleByNameHandler {
    role_service: Arc<dyn IRoleService>,
}

impl GetRoleByNameHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetRoleByNameQuery, Role> for GetRoleByNameHandler {
    async fn handle(&self, query: GetRoleByNameQuery) -> AppResult<Role> {
        self.role_service.get_role_by_name(query).await
    }
}
