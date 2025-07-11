#[rustfmt::skip]
use crate::{
    CommandHandler,
    interfaces::user_service::IUserService,
    commands::user::create_user_command::CreateUserCommand,
};
use std::sync::Arc;
use tradewinds_domain::entities::user::User;
use tradewinds_error::AppResult;

/// 创建用户命令处理器
///
/// 参数：
/// - user_service: 用户服务
///
/// 返回：
/// - 创建用户命令处理器
pub struct CreateUserHandler {
    user_service: Arc<dyn IUserService>,
}

impl CreateUserHandler {
    pub fn new(user_service: Arc<dyn IUserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait::async_trait]
impl CommandHandler<CreateUserCommand, User> for CreateUserHandler {
    async fn handle(&self, command: CreateUserCommand) -> AppResult<User> {
        self.user_service.create_user(command).await
    }
}
