use crate::persistence::repositories::SeaOrmSystemSettingRepository;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tradewinds_application::interfaces::system_setting_service::ISystemSettingService;
use tradewinds_application::services::system_setting_service::SystemSettingService;
use tradewinds_domain::repositories::system_setting_repository::SystemSettingRepository;

pub struct SystemSettingServiceBundle {
    pub service: Arc<dyn ISystemSettingService>,
    pub system_setting_repo: Arc<dyn SystemSettingRepository>,
}

pub fn init_system_setting_service(db: &DatabaseConnection) -> SystemSettingServiceBundle {
    let system_setting_repo: Arc<dyn SystemSettingRepository> =
        Arc::new(SeaOrmSystemSettingRepository::new(db.clone()));
    let service = Arc::new(SystemSettingService::new(system_setting_repo.clone())) as Arc<dyn ISystemSettingService>;
    SystemSettingServiceBundle { service, system_setting_repo }
}
