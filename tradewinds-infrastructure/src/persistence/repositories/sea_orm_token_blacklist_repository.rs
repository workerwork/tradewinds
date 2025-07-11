use crate::persistence::entities::token_blacklist;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tradewinds_domain::repositories::TokenBlacklistRepository;
use tradewinds_domain::value_objects::{auth::auth_token::Token, user::UserId};
use tradewinds_error::{AppError, AppResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct SeaOrmTokenBlacklistRepository {
    db: DatabaseConnection,
}

impl SeaOrmTokenBlacklistRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TokenBlacklistRepository for SeaOrmTokenBlacklistRepository {
    async fn add(&self, token: &Token, user_id: &UserId, expires_at: i64) -> AppResult<()> {
        let expires_at_dt = chrono::NaiveDateTime::from_timestamp_opt(expires_at, 0)
            .map(|ndt| chrono::DateTime::<Utc>::from_utc(ndt, Utc))
            .ok_or_else(|| AppError::DatabaseError("无效的 expires_at 时间戳".to_string()))?
            .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

        let model = token_blacklist::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            jti: Set(token.value().to_string()),
            user_id: Set(user_id.to_string()),
            expires_at: Set(expires_at_dt),
            created_at: sea_orm::ActiveValue::NotSet,
        };
        let res = model.insert(&self.db).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("None of the records are inserted") {
                    // 实际已插入，兼容 SeaORM bug
                    Ok(())
                } else {
                    Err(AppError::DatabaseError(msg))
                }
            }
        }
    }

    async fn is_blacklisted(&self, token: &Token) -> AppResult<bool> {
        token_blacklist::Entity::find()
            .filter(token_blacklist::Column::Jti.eq(token.value()))
            .one(&self.db)
            .await
            .map(|o| o.is_some())
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn cleanup(&self) -> AppResult<()> {
        // Implementation for cleanup
        Ok(())
    }
}
