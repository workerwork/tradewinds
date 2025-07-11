#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::user_service::IUserService,
    commands::user::delete_user_command::DeleteUserCommand,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 删除用户命令处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 删除用户命令处理器
pub struct DeleteUserHandler {
    user_service: Arc<dyn IUserService>,
}

impl DeleteUserHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<DeleteUserCommand, ()> for DeleteUserHandler {
    async fn handle(&self, command: DeleteUserCommand) -> AppResult<()> {
        self.user_service.delete_user(command).await
    }
}
