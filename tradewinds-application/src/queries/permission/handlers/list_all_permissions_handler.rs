use crate::queries::auth::user_info::PermissionInfo;
use crate::{
    QueryHandler, interfaces::permission_service::IPermissionService,
    queries::permission::list_all_permissions_query::ListAllPermissionsQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

// 移除 PermissionInfo 相关内容和 build_permission_tree 函数

pub struct ListAllPermissionsHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl ListAllPermissionsHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<ListAllPermissionsQuery, Vec<Permission>> for ListAllPermissionsHandler {
    async fn handle(&self, _query: ListAllPermissionsQuery) -> AppResult<Vec<Permission>> {
        self.permission_service.list_all_permissions().await
    }
}
