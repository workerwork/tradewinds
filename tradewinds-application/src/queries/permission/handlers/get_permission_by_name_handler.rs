#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::permission_service::IPermissionService,
    queries::permission::get_permission_by_name_query::GetPermissionByNameQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 根据权限名称查询权限查询处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 根据权限名称查询权限查询处理器
pub struct GetPermissionByNameHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl GetPermissionByNameHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetPermissionByNameQuery, Permission> for GetPermissionByNameHandler {
    async fn handle(&self, query: GetPermissionByNameQuery) -> AppResult<Permission> {
        self.permission_service.get_permission_by_name(query).await
    }
}
