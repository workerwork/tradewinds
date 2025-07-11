use crate::persistence::repositories::{
    SeaOrmRoleRepository, SeaOrmTokenBlacklistRepository, SeaOrmUserAggregateRepository, SeaOrmUserRepository,
    SeaOrmUserRoleRepository,
};
use crate::services::auth::bcrypt_password_service::BcryptPasswordService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tradewinds_application::interfaces::user_service::IUserService;
use tradewinds_application::services::user_service::UserService;
use tradewinds_domain::repositories::{
    RoleRepository, SystemSettingRepository, UserAggregateRepository, UserRepository, UserRoleRepository,
};
use tradewinds_domain::services::PasswordService;

pub struct UserServiceBundle {
    pub service: Arc<dyn IUserService>,
    pub user_repo: Arc<dyn UserRepository>,
    pub user_agg_repo: Arc<dyn UserAggregateRepository>,
    pub user_role_repo: Arc<dyn UserRoleRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
}

pub fn init_user_service(
    db: &DatabaseConnection,
    system_setting_repo: Arc<dyn SystemSettingRepository>,
) -> UserServiceBundle {
    let user_repo: Arc<dyn UserRepository> = Arc::new(SeaOrmUserRepository::new(db.clone()));
    let user_agg_repo: Arc<dyn UserAggregateRepository> = Arc::new(SeaOrmUserAggregateRepository::new(db.clone()));
    let user_role_repo: Arc<dyn UserRoleRepository> = Arc::new(SeaOrmUserRoleRepository::new(db.clone()));
    let role_repo: Arc<dyn RoleRepository> = Arc::new(SeaOrmRoleRepository::new(db.clone()));
    let password_service = Arc::new(BcryptPasswordService::new()) as Arc<dyn PasswordService>;
    let service = Arc::new(UserService::new(
        user_agg_repo.clone(),
        user_repo.clone(),
        role_repo.clone(),
        user_role_repo.clone(),
        password_service,
        system_setting_repo.clone(),
    )) as Arc<dyn IUserService>;
    UserServiceBundle { service, user_repo, user_agg_repo, user_role_repo, role_repo }
}
