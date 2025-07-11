// 标准库 & 三方库
use std::sync::Arc;
use tradewinds_domain::services::{PasswordService, TokenService};

// 应用层接口与服务
use tradewinds_application::{
    interfaces::{
        auth_service::IAuthService, permission_service::IPermissionService, role_service::IRoleService,
        system_setting_service::ISystemSettingService, user_service::IUserService,
    },
    services::{
        auth_service::AuthService, permission_service::PermissionService, role_service::RoleService,
        system_setting_service::SystemSettingService, user_service::UserService,
    },
};

// 配置与数据库
use crate::config::AppConfig;
use crate::persistence::repositories::{
    SeaOrmPermissionAggregateRepository, SeaOrmPermissionRepository, SeaOrmRoleAggregateRepository,
    SeaOrmRoleRepository, SeaOrmSystemSettingRepository, SeaOrmTokenBlacklistRepository, SeaOrmUserAggregateRepository,
    SeaOrmUserRepository, SeaOrmUserRoleRepository,
};

// 领域层 repository
use tradewinds_domain::repositories::{
    PermissionAggregateRepository, PermissionRepository, RoleAggregateRepository, RolePermissionRepository,
    RoleRepository, TokenBlacklistRepository, UserAggregateRepository, UserRepository, UserRoleRepository,
    system_setting_repository::SystemSettingRepository,
};

// 基础设施服务
use crate::services::auth::bcrypt_password_service::BcryptPasswordService;
use crate::services::auth::jwt_token_service::JwtTokenService;

pub use crate::di;

pub async fn init_application_service(
    config: &AppConfig,
) -> anyhow::Result<(
    Arc<dyn IAuthService>,
    Arc<dyn IUserService>,
    Arc<dyn IRoleService>,
    Arc<dyn IPermissionService>,
    Arc<dyn TokenService>,
    Arc<dyn ISystemSettingService>,
)> {
    use sea_orm::Database;
    let db = Database::connect(&config.database_url).await?;

    let system_setting_service_bundle = di::system_setting_di::init_system_setting_service(&db);
    let user_service_bundle =
        di::user_di::init_user_service(&db, system_setting_service_bundle.system_setting_repo.clone());
    let role_service_bundle = di::role_di::init_role_service(&db);
    let permission_service_bundle = di::permission_di::init_permission_service(&db);

    let token_blacklist_repo = di::auth_di::init_token_blacklist_repo(&db);
    let jwt_token_service =
        Arc::new(JwtTokenService::new(config.clone(), token_blacklist_repo)) as Arc<dyn TokenService>;
    let bcrypt_password_service = Arc::new(BcryptPasswordService::new()) as Arc<dyn PasswordService>;
    let auth_service: Arc<dyn IAuthService> = Arc::new(AuthService::new(
        user_service_bundle.user_repo.clone(),
        role_service_bundle.role_repo.clone(),
        permission_service_bundle.permission_repo.clone(),
        user_service_bundle.user_role_repo.clone(),
        user_service_bundle.user_agg_repo.clone(),
        jwt_token_service.clone(),
        bcrypt_password_service.clone(),
    ));

    Ok((
        auth_service,
        user_service_bundle.service.clone(),
        role_service_bundle.service.clone(),
        permission_service_bundle.service.clone(),
        jwt_token_service,
        system_setting_service_bundle.service.clone(),
    ))
}
