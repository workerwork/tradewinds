#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::role_service::IRoleService,
    commands::role::revoke_permission_command::RevokePermissionCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 撤销权限给角色命令处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 撤销权限给角色命令处理器
pub struct RevokePermissionHandler {
    role_service: Arc<dyn IRoleService>,
}

impl RevokePermissionHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<RevokePermissionCommand, ()> for RevokePermissionHandler {
    async fn handle(&self, command: RevokePermissionCommand) -> AppResult<()> {
        self.role_service.revoke_permission(command).await
    }
}
