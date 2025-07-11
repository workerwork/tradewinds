#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::role_service::IRoleService,
    commands::role::update_role_command::UpdateRoleCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 更新角色命令处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 更新角色命令处理器
pub struct UpdateRoleHandler {
    role_service: Arc<dyn IRoleService>,
}

impl UpdateRoleHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<UpdateRoleCommand, ()> for UpdateRoleHandler {
    async fn handle(&self, command: UpdateRoleCommand) -> AppResult<()> {
        self.role_service.update_role(command).await
    }
}
