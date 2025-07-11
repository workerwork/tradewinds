#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::role_service::IRoleService,
    commands::role::delete_role_command::DeleteRoleCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 删除角色命令处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 删除角色命令处理器
pub struct DeleteRoleHandler {
    role_service: Arc<dyn IRoleService>,
}

impl DeleteRoleHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<DeleteRoleCommand, ()> for DeleteRoleHandler {
    async fn handle(&self, command: DeleteRoleCommand) -> AppResult<()> {
        self.role_service.delete_role(command).await
    }
}
