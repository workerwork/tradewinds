#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::user_service::IUserService,
    commands::user::assign_role_command::AssignRoleCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 分配角色给用户命令处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 分配角色给用户命令处理器
pub struct AssignRoleHandler {
    user_service: Arc<dyn IUserService>,
}

impl AssignRoleHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<AssignRoleCommand, ()> for AssignRoleHandler {
    async fn handle(&self, command: AssignRoleCommand) -> AppResult<()> {
        self.user_service.assign_role(command).await
    }
}
