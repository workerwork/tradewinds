#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::role_service::IRoleService,
    commands::role::assign_permission_command::AssignPermissionCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 分配权限给角色命令处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 分配权限给角色命令处理器
pub struct AssignPermissionHandler {
    role_service: Arc<dyn IRoleService>,
}

impl AssignPermissionHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<AssignPermissionCommand, ()> for AssignPermissionHandler {
    async fn handle(&self, command: AssignPermissionCommand) -> AppResult<()> {
        self.role_service.assign_permission(command).await
    }
}
