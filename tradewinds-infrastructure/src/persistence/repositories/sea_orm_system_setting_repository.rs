use crate::persistence::entities::system_setting::{ActiveModel, Column, Entity};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tradewinds_domain::entities::system_setting::SystemSetting;
use tradewinds_domain::repositories::system_setting_repository::SystemSettingRepository;
use tradewinds_domain::value_objects::system_setting::{SystemSettingId, SystemSettingKey, SystemSettingValue};
use tradewinds_error::AppResult;

#[derive(Clone)]
pub struct SeaOrmSystemSettingRepository {
    pub db: DatabaseConnection,
}

impl SeaOrmSystemSettingRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl SystemSettingRepository for SeaOrmSystemSettingRepository {
    async fn get_by_key(&self, key: &SystemSettingKey) -> AppResult<Option<SystemSetting>> {
        let result = Entity::find().filter(Column::Key.eq(key.value())).one(&self.db).await?;
        Ok(result.map(|m| SystemSetting {
            id: SystemSettingId::new(m.id).unwrap(),
            key: SystemSettingKey::new(m.key).unwrap(),
            value: SystemSettingValue::new(m.value).unwrap(),
            description: m.description,
            updated_at: m.updated_at.naive_utc(),
        }))
    }

    async fn set_value(&self, key: &SystemSettingKey, value: &SystemSettingValue) -> AppResult<()> {
        use sea_orm::ActiveValue::Set as AVSet;
        let setting = Entity::find().filter(Column::Key.eq(key.value())).one(&self.db).await?;
        if let Some(m) = setting {
            let mut am: ActiveModel = m.into();
            am.value = AVSet(value.value().to_string());
            am.update(&self.db).await?;
        }
        Ok(())
    }
}
