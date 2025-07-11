#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::role_service::IRoleService,
    queries::role::list_roles_query::ListRolesQuery,
};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::{permission::Permission, role::Role};
use tradewinds_error::AppResult;

/// 查询角色列表查询处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 查询角色列表查询处理器
pub struct ListRolesHandler {
    role_service: Arc<dyn IRoleService>,
}

impl ListRolesHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListRolesQuery, PaginatedResult<(Role, Vec<Permission>)>> for ListRolesHandler {
    async fn handle(&self, query: ListRolesQuery) -> AppResult<PaginatedResult<(Role, Vec<Permission>)>> {
        self.role_service.list_roles(query).await
    }
}
