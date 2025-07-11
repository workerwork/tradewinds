#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::permission_service::IPermissionService,
    queries::permission::list_permissions_by_type_query::ListPermissionsByTypeQuery,
};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 根据权限类型查询权限列表查询处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 根据权限类型查询权限列表查询处理器
pub struct ListPermissionsByTypeHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl ListPermissionsByTypeHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListPermissionsByTypeQuery, PaginatedResult<Permission>> for ListPermissionsByTypeHandler {
    async fn handle(&self, query: ListPermissionsByTypeQuery) -> AppResult<PaginatedResult<Permission>> {
        self.permission_service.list_permissions_by_type(query).await
    }
}
