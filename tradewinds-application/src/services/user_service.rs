#[rustfmt::skip]
use crate::{
    commands::user::*,
    queries::user::*,
    interfaces::IUserService,
};
#[rustfmt::skip]
use tradewinds_domain::{
    aggregates::user_aggregate::UserAggregate,
    entities::{
        permission::Permission, 
        role::Role, 
        user::User
    },
    repositories::{
        RoleRepository, SystemSettingRepository, UserAggregateRepository, UserRepository, UserRoleRepository
    },
    services::auth::PasswordService,
    value_objects::auth::auth_password::Password,
    value_objects::user::user_id::UserId,
    value_objects::user::Email,
    value_objects::user::Phone,
    value_objects::role_permission::role_permission_id::RolePermissionId,
    value_objects::auth::auth_username::AuthUsername,
    value_objects::user::UserStatus,
    value_objects::system_setting::{SystemSettingKey, SystemSettingValue},
};

use crate::queries::system_setting::get_system_setting_query::GetSystemSettingQuery;
use std::sync::Arc;
use tradewinds_common::PaginatedResult;
use tradewinds_domain::value_objects::RoleId;
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct UserService {
    user_agg_repo: Arc<dyn UserAggregateRepository>,
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    user_role_repo: Arc<dyn UserRoleRepository>,
    password_service: Arc<dyn PasswordService>,
    system_setting_repo: Arc<dyn SystemSettingRepository>,
}

impl UserService {
    pub fn new(
        user_agg_repo: Arc<dyn UserAggregateRepository>,
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        user_role_repo: Arc<dyn UserRoleRepository>,
        password_service: Arc<dyn PasswordService>,
        system_setting_repo: Arc<dyn SystemSettingRepository>,
    ) -> Self {
        Self { user_agg_repo, user_repo, role_repo, user_role_repo, password_service, system_setting_repo }
    }
}

#[async_trait::async_trait]
impl IUserService for UserService {
    async fn list_users(&self, query: ListUsersQuery) -> AppResult<PaginatedResult<(User, Vec<Role>)>> {
        let (limit, offset) = query.pagination();
        let username_obj = query.username.as_ref().map(|s| AuthUsername::new(s.clone())).transpose()?;
        let username_ref = username_obj.as_ref();
        let phone_ref = query.phone.as_deref();
        let email_obj = query.email.as_ref().map(|s| Email::new(s.clone())).transpose()?;
        let email_ref = email_obj.as_ref();
        let status_obj = query.status.map(UserStatus::from_i32).transpose()?;
        let (users, total) = self
            .user_repo
            .search(username_ref, phone_ref, email_ref, status_obj, query.show_deleted, limit, offset)
            .await?;

        // 为每个用户查询角色信息
        let mut users_with_roles = Vec::new();
        for user in users {
            let user_roles = self.user_role_repo.find_by_user_id(&user.id).await?;
            let role_ids: Vec<_> = user_roles.iter().map(|ur| ur.role_id.clone()).collect();

            let roles = if !role_ids.is_empty() { self.role_repo.find_by_ids(&role_ids).await? } else { Vec::new() };

            users_with_roles.push((user, roles));
        }

        Ok(PaginatedResult { items: users_with_roles, total })
    }

    async fn create_user(&self, cmd: CreateUserCommand) -> AppResult<User> {
        if self.user_repo.find_by_username(&cmd.username).await?.is_some() {
            return Err(AppError::Validation("Username already exists".into()));
        }

        if self.user_repo.find_by_email(&cmd.email).await?.is_some() {
            return Err(AppError::Validation("Email already exists".into()));
        }

        let hashed_password = self.password_service.hash(cmd.password.as_ref()).await?;
        let hashed_password = Password::new(hashed_password)?;

        // 如果提供了角色ID，验证这些角色是否存在
        let role_ids = if let Some(role_ids) = cmd.role_ids {
            // 验证角色是否存在
            let existing_roles = self.role_repo.find_by_ids(&role_ids).await?;
            if existing_roles.len() != role_ids.len() {
                return Err(AppError::Validation("Some roles do not exist".into()));
            }
            role_ids
        } else {
            Vec::new()
        };

        let user_agg = if role_ids.is_empty() {
            UserAggregate::create(cmd.username, cmd.email, hashed_password, cmd.real_name, cmd.phone, cmd.avatar)?
        } else {
            UserAggregate::create_with_roles(
                cmd.username,
                cmd.email,
                hashed_password,
                cmd.real_name,
                cmd.phone,
                cmd.avatar,
                role_ids,
            )?
        };

        self.user_agg_repo.create(&user_agg).await?;

        Ok(user_agg.user)
    }

    async fn update_user(&self, cmd: UpdateUserCommand) -> AppResult<()> {
        let mut user_agg =
            self.user_agg_repo.find_by_id(&cmd.id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;

        user_agg.update(cmd.real_name, cmd.phone, cmd.avatar, cmd.status, cmd.email, cmd.role_ids)?;

        self.user_agg_repo.save(&user_agg).await?;

        Ok(())
    }

    async fn delete_user(&self, cmd: DeleteUserCommand) -> AppResult<()> {
        self.user_agg_repo.delete_by_id(&cmd.id).await?;
        Ok(())
    }

    async fn reset_password(&self, cmd: ResetPasswordCommand) -> AppResult<()> {
        let mut user_agg =
            self.user_agg_repo.find_by_id(&cmd.id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let default_password_vo = self
            .system_setting_repo
            .get_by_key(&SystemSettingKey::new("default_password".to_string()).unwrap())
            .await?
            .map(|setting| setting.value)
            .filter(|v| !v.value().trim().is_empty())
            .unwrap_or_else(|| SystemSettingValue::new("123456".to_string()).unwrap());

        // 先 hash 明文密码
        let hashed_password = self.password_service.hash(default_password_vo.value()).await?;
        let password = Password::new(hashed_password)?;
        user_agg.reset_password(password);
        self.user_agg_repo.save(&user_agg).await?;

        Ok(())
    }
    async fn assign_role(&self, cmd: AssignRoleCommand) -> AppResult<()> {
        let mut user_agg = self
            .user_agg_repo
            .find_by_id(&cmd.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        user_agg.assign_role(&cmd.role_id)?;

        self.user_agg_repo.save(&user_agg).await?;
        Ok(())
    }

    async fn revoke_role(&self, cmd: RevokeRoleCommand) -> AppResult<()> {
        let mut user_agg = self
            .user_agg_repo
            .find_by_id(&cmd.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        user_agg.revoke_role(&cmd.role_id)?;

        self.user_agg_repo.save(&user_agg).await?;
        Ok(())
    }

    async fn get_user_by_id(&self, query: GetUserByIdQuery) -> AppResult<User> {
        self.user_repo.find_by_id(&query.user_id).await?.ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    async fn get_user_by_username(&self, query: GetUserByUsernameQuery) -> AppResult<User> {
        self.user_repo
            .find_by_username(&query.username)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    async fn get_user_by_email(&self, query: GetUserByEmailQuery) -> AppResult<User> {
        self.user_repo
            .find_by_email(&query.user_email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    async fn get_user_roles(&self, query: GetUserRolesQuery) -> AppResult<Vec<Role>> {
        let user_roles = self.user_role_repo.find_by_user_id(&query.user_id).await?;

        let role_ids: Vec<_> = user_roles.iter().map(|ur| ur.role_id.clone()).collect();

        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        let roles = self.role_repo.find_by_ids(&role_ids).await?;

        Ok(roles)
    }

    async fn get_user_permissions(&self, query: GetUserPermissionsQuery) -> AppResult<Vec<Permission>> {
        // 1. 获取用户所有角色
        let roles = self.user_role_repo.find_by_user_id(&query.user_id).await?;
        if roles.is_empty() {
            return Ok(Vec::new());
        }
        // 2. 提取所有角色ID
        let role_ids: Vec<RoleId> = roles.iter().map(|r| RoleId::new(r.role_id.value().to_string()).unwrap()).collect();
        // 3. 查询这些角色拥有的所有权限
        let mut all_permissions = self.role_repo.find_permissions_by_ids(&role_ids).await?;

        // 4. 去重
        all_permissions.sort_by_key(|p| p.id.clone());
        all_permissions.dedup_by_key(|a| a.id.clone());

        Ok(all_permissions)
    }
}
