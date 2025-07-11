#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::role_service::IRoleService,
    queries::role::get_role_permissions_query::GetRolePermissionsQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 根据角色ID获取角色权限查询处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 根据角色ID获取角色权限查询处理器
pub struct GetRolePermissionsHandler {
    role_service: Arc<dyn IRoleService>,
}

impl GetRolePermissionsHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetRolePermissionsQuery, Vec<Permission>> for GetRolePermissionsHandler {
    async fn handle(&self, query: GetRolePermissionsQuery) -> AppResult<Vec<Permission>> {
        self.role_service.get_role_permissions(query).await
    }
}
