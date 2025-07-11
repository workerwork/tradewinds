use crate::{
    commands::auth::*, interfaces::auth_service::IAuthService, queries::auth::user_info::CurrentUserInfo,
    queries::auth::*,
};
use std::sync::Arc;
use tradewinds_domain::{
    aggregates::user_aggregate::UserAggregate,
    repositories::{PermissionRepository, RoleRepository, UserAggregateRepository, UserRepository, UserRoleRepository},
    services::auth::{PasswordService, TokenService},
    value_objects::auth::{auth_password::Password, auth_token::Token},
};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    permission_repo: Arc<dyn PermissionRepository>,
    user_role_repo: Arc<dyn UserRoleRepository>,
    user_agg_repo: Arc<dyn UserAggregateRepository>,
    token_service: Arc<dyn TokenService>,
    password_service: Arc<dyn PasswordService>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        permission_repo: Arc<dyn PermissionRepository>,
        user_role_repo: Arc<dyn UserRoleRepository>,
        user_agg_repo: Arc<dyn UserAggregateRepository>,
        token_service: Arc<dyn TokenService>,
        password_service: Arc<dyn PasswordService>,
    ) -> Self {
        Self { user_repo, role_repo, permission_repo, user_role_repo, user_agg_repo, token_service, password_service }
    }
}

#[async_trait::async_trait]
impl IAuthService for AuthService {
    /// 注册用户
    async fn register(&self, cmd: RegisterCommand) -> AppResult<()> {
        // 检查用户名是否已存在
        if self.user_repo.exists_by_username(&cmd.username).await? {
            return Err(AppError::Validation("Username already exists".into()));
        }

        // 检查邮箱是否已存在
        if self.user_repo.exists_by_email(&cmd.email).await? {
            return Err(AppError::Validation("Email already exists".into()));
        }

        // 哈希密码
        let hashed_password = self.password_service.hash(cmd.password.value()).await?;
        let hashed_password = Password::new(hashed_password)?;

        // 创建用户聚合
        let user_agg = UserAggregate::create(
            cmd.username,
            cmd.email,
            hashed_password,
            cmd.real_name,
            cmd.phone,
            None, // No avatar on registration
        )?;

        // 用户聚合落库
        self.user_agg_repo.create(&user_agg).await
    }

    /// 登录
    async fn login(&self, cmd: LoginCommand) -> AppResult<Token> {
        // 统一的登录失败错误
        let invalid_cred = || AppError::Validation("Invalid username or password".into());

        // 查询用户
        let user = self.user_repo.find_by_username(&cmd.username).await?.ok_or_else(&invalid_cred)?;

        // 新增：检查用户状态
        if !user.status.is_active() {
            return Err(AppError::Validation("用户未启用或已被禁用/删除，无法登录".into()));
        }

        // 验证密码
        let valid = self.password_service.verify(&user.password.value(), cmd.password.value()).await.unwrap_or(false);
        if !valid {
            return Err(invalid_cred());
        }

        // 生成令牌
        let token = self.token_service.generate(&user.id).await?;

        Ok(token)
    }

    /// 登出
    async fn logout(&self, cmd: LogoutCommand) -> AppResult<()> {
        // 验证令牌
        let token_data = self.token_service.validate(&cmd.token).await?;

        // 拉黑令牌
        self.token_service.revoke(&cmd.token).await?;

        // 可选：记录登出事件
        // self.event_bus.publish(UserLoggedOutEvent { user_id: token_data.user_id }).await?;

        Ok(())
    }

    /// 修改密码
    async fn change_password(&self, cmd: ChangePasswordCommand) -> AppResult<()> {
        // 验证令牌
        let token_claims = self.token_service.validate(&cmd.token).await?;

        // 查询用户聚合
        let mut user_agg = self
            .user_agg_repo
            .find_by_id(&token_claims.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User not found: {}", token_claims.user_id)))?;

        // 验证旧密码
        self.password_service
            .verify(&user_agg.user.password.value(), cmd.old_password.value())
            .await
            .map_err(|_| AppError::Validation("Invalid old password".into()))?;

        // 哈希新密码并更新
        let new_hashed_password = self.password_service.hash(cmd.new_password.value()).await?;
        user_agg.user.password = Password::new(new_hashed_password)?;

        // 保存用户聚合
        self.user_agg_repo.save(&user_agg).await?;

        // 可选：记录密码修改事件
        // self.event_bus.publish(UserPasswordChangedEvent { user_id: user_agg.user.id.clone() }).await?;

        Ok(())
    }

    /// 获取当前用户
    async fn get_current_user(&self, query: GetCurrentUserQuery) -> AppResult<CurrentUserInfo> {
        // 验证令牌
        let claims = self.token_service.validate(&query.token).await?;

        // 查询用户
        let user = self
            .user_repo
            .find_by_id(&claims.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User not found: {}", claims.user_id)))?;

        // 查询角色
        let roles = self.user_role_repo.find_by_user_id(&user.id).await?;
        let role_ids = roles.iter().map(|r| r.role_id.clone()).collect::<Vec<_>>();
        let roles = self.role_repo.find_by_ids(&role_ids).await?;

        // 查询权限
        let permissions = self.permission_repo.find_by_user_id(&user.id).await?;

        Ok(CurrentUserInfo {
            user: user.into(),
            roles: roles.into_iter().map(Into::into).collect(),
            permissions: permissions.into_iter().map(Into::into).collect(),
        })
    }
}
