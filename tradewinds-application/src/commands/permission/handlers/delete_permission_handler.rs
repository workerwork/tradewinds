#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::permission_service::IPermissionService,
    commands::permission::delete_permission_command::DeletePermissionCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 删除权限命令处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 删除权限命令处理器
pub struct DeletePermissionHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl DeletePermissionHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<DeletePermissionCommand, ()> for DeletePermissionHandler {
    async fn handle(&self, command: DeletePermissionCommand) -> AppResult<()> {
        self.permission_service.delete_permission(command).await
    }
}
