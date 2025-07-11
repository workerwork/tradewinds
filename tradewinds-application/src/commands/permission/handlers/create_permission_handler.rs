#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::permission_service::IPermissionService,
    commands::permission::create_permission_command::CreatePermissionCommand,
};
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

/// 创建权限命令处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 创建权限命令处理器
pub struct CreatePermissionHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl CreatePermissionHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<CreatePermissionCommand, Permission> for CreatePermissionHandler {
    async fn handle(&self, command: CreatePermissionCommand) -> AppResult<Permission> {
        self.permission_service.create_permission(command).await
    }
}
