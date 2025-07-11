#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::permission_service::IPermissionService,
    queries::permission::list_permissions_query::ListPermissionsQuery,
};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 查询权限列表查询处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 查询权限列表查询处理器
pub struct ListPermissionsHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl ListPermissionsHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListPermissionsQuery, PaginatedResult<Permission>> for ListPermissionsHandler {
    async fn handle(&self, query: ListPermissionsQuery) -> AppResult<PaginatedResult<Permission>> {
        self.permission_service.list_permissions(query).await
    }
}
