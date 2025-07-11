#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::permission_service::IPermissionService,
    commands::permission::update_permission_command::UpdatePermissionCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 更新权限命令处理器
///
/// 参数：
/// - permission_service: 权限服务
///
/// 返回：
/// - 更新权限命令处理器
pub struct UpdatePermissionHandler {
    permission_service: Arc<dyn IPermissionService>,
}

impl UpdatePermissionHandler {
    pub fn new(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self { permission_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<UpdatePermissionCommand, ()> for UpdatePermissionHandler {
    async fn handle(&self, command: UpdatePermissionCommand) -> AppResult<()> {
        self.permission_service.update_permission(command).await
    }
}
