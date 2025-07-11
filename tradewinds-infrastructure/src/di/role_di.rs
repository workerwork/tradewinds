use crate::persistence::repositories::{SeaOrmRoleAggregateRepository, SeaOrmRoleRepository};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tradewinds_application::interfaces::role_service::IRoleService;
use tradewinds_application::services::role_service::RoleService;
use tradewinds_domain::repositories::{RoleAggregateRepository, RoleRepository};

pub struct RoleServiceBundle {
    pub service: Arc<dyn IRoleService>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub role_agg_repo: Arc<dyn RoleAggregateRepository>,
}

pub fn init_role_service(db: &DatabaseConnection) -> RoleServiceBundle {
    let role_repo: Arc<dyn RoleRepository> = Arc::new(SeaOrmRoleRepository::new(db.clone()));
    let role_agg_repo: Arc<dyn RoleAggregateRepository> = Arc::new(SeaOrmRoleAggregateRepository::new(db.clone()));
    let service = Arc::new(RoleService::new(role_repo.clone(), role_agg_repo.clone())) as Arc<dyn IRoleService>;
    RoleServiceBundle { service, role_repo, role_agg_repo }
}
