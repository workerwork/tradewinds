use crate::persistence::repositories::{SeaOrmPermissionAggregateRepository, SeaOrmPermissionRepository};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tradewinds_application::interfaces::permission_service::IPermissionService;
use tradewinds_application::services::permission_service::PermissionService;
use tradewinds_domain::repositories::{PermissionAggregateRepository, PermissionRepository};

pub struct PermissionServiceBundle {
    pub service: Arc<dyn IPermissionService>,
    pub permission_repo: Arc<dyn PermissionRepository>,
    pub permission_agg_repo: Arc<dyn PermissionAggregateRepository>,
}

pub fn init_permission_service(db: &DatabaseConnection) -> PermissionServiceBundle {
    let permission_repo: Arc<dyn PermissionRepository> = Arc::new(SeaOrmPermissionRepository::new(db.clone()));
    let permission_agg_repo: Arc<dyn PermissionAggregateRepository> =
        Arc::new(SeaOrmPermissionAggregateRepository::new(db.clone()));
    let service = Arc::new(PermissionService::new(permission_repo.clone(), permission_agg_repo.clone()))
        as Arc<dyn IPermissionService>;
    PermissionServiceBundle { service, permission_repo, permission_agg_repo }
}
