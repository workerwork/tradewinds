#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::permission_service::IPermissionService,
    queries::permission::list_permissions_by_parent_id_query::ListPermissionsByParentIdQuery,
};
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 根据父权限ID查询权限列表查询处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 根据父权限ID查询权限列表查询处理器
pub struct ListPermissionsByParentIdHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl ListPermissionsByParentIdHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListPermissionsByParentIdQuery, PaginatedResult<Permission>> for ListPermissionsByParentIdHandler {
    async fn handle(&self, query: ListPermissionsByParentIdQuery) -> AppResult<PaginatedResult<Permission>> {
        self.permission_service.list_permissions_by_parent_id(query).await
    }
}
