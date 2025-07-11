#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::user_service::IUserService,
    commands::user::revoke_role_command::RevokeRoleCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 撤销用户角色命令处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 撤销用户角色命令处理器
pub struct RevokeRoleHandler {
    user_service: Arc<dyn IUserService>,
}

impl RevokeRoleHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<RevokeRoleCommand, ()> for RevokeRoleHandler {
    async fn handle(&self, command: RevokeRoleCommand) -> AppResult<()> {
        self.user_service.revoke_role(command).await
    }
}
