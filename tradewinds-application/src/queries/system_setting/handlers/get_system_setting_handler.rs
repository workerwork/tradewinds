#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::system_setting_service::ISystemSettingService,
    queries::system_setting::get_system_setting_query::GetSystemSettingQuery,
};
use std::sync::Arc;
use tradewinds_domain::entities::system_setting::SystemSetting;
use tradewinds_error::{AppError, AppResult};

/// 根据系统设置键获取系统设置查询处理器
///
/// 参数：
/// - system_setting_service: 系统设置服务
///
/// 返回：
/// - 根据系统设置键获取系统设置查询处理器
pub struct GetSystemSettingHandler {
    pub system_setting_service: Arc<dyn ISystemSettingService>,
}

impl GetSystemSettingHandler {
    pub fn new(system_setting_service: Arc<dyn ISystemSettingService>) -> Self {
        Self { system_setting_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetSystemSettingQuery, SystemSetting> for GetSystemSettingHandler {
    async fn handle(&self, query: GetSystemSettingQuery) -> AppResult<SystemSetting> {
        match self.system_setting_service.get_by_key(query).await? {
            Some(setting) => Ok(setting),
            None => Err(AppError::NotFound("Setting not found".to_string())),
        }
    }
}
