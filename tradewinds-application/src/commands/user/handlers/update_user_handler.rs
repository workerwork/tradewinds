#[rustfmt::skip]
use crate::{
    interfaces::user_service::IUserService,
    commands::user::update_user_command::UpdateUserCommand,
    CommandHandler,
};
use std::sync::Arc;
use tradewinds_error::AppResult;

/// 更新用户命令处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 更新用户命令处理器
pub struct UpdateUserHandler {
    user_service: Arc<dyn IUserService>,
}

impl UpdateUserHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<UpdateUserCommand, ()> for UpdateUserHandler {
    async fn handle(&self, command: UpdateUserCommand) -> AppResult<()> {
        self.user_service.update_user(command).await
    }
}
