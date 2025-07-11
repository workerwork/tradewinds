#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::permission_service::IPermissionService,
    queries::permission::get_permission_by_id_query::GetPermissionByIdQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 根据权限ID查询权限查询处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 根据权限ID查询权限查询处理器
pub struct GetPermissionByIdHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl GetPermissionByIdHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetPermissionByIdQuery, Permission> for GetPermissionByIdHandler {
    async fn handle(&self, query: GetPermissionByIdQuery) -> AppResult<Permission> {
        self.permission_service.get_permission_by_id(query).await
    }
}
