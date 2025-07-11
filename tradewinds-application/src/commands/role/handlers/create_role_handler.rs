#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::role_service::IRoleService,
    commands::role::create_role_command::CreateRoleCommand,
};
use std::sync::Arc;
use tradewinds_domain::entities::role::Role;
use tradewinds_error::AppResult;

/// 创建角色命令处理器
///
/// 参数：
/// - role_service: 角色服务
///
/// 返回：
/// - 创建角色命令处理器
pub struct CreateRoleHandler {
    role_service: Arc<dyn IRoleService>,
}

impl CreateRoleHandler {
    pub fn new(role_service: Arc<dyn IRoleService>) -> Self {
        Self { role_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<CreateRoleCommand, Role> for CreateRoleHandler {
    async fn handle(&self, command: CreateRoleCommand) -> AppResult<Role> {
        self.role_service.create_role(command).await
    }
}
