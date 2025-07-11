use async_trait::async_trait;
use redis::{Client, Commands, Connection};
use serde::{Serialize, de::DeserializeOwned};

use tradewinds_error::{AppError, AppResult};

#[async_trait]
pub trait Cache: Send + Sync {
    async fn set<T: Serialize + Send + Sync>(&self, key: &str, value: &T, ttl_seconds: u64) -> AppResult<()>;
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> AppResult<Option<T>>;
    async fn delete(&self, key: &str) -> AppResult<()>;
}

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> AppResult<Self> {
        let client = Client::open(redis_url).map_err(|e| AppError::System(format!("Redis connection error: {}", e)))?;
        Ok(Self { client })
    }

    fn get_conn(&self) -> AppResult<Connection> {
        self.client.get_connection().map_err(|e| AppError::System(format!("Redis connection error: {}", e)))
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn set<T: Serialize + Send + Sync>(&self, key: &str, value: &T, ttl_seconds: u64) -> AppResult<()> {
        let mut conn = self.get_conn()?;
        let serialized =
            serde_json::to_string(value).map_err(|e| AppError::System(format!("Serialization error: {}", e)))?;

        let _: () = conn
            .set_ex(key, serialized, ttl_seconds)
            .map_err(|e| AppError::System(format!("Redis set error: {}", e)))?;
        Ok(())
    }

    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> AppResult<Option<T>> {
        let mut conn = self.get_conn()?;
        let value: Option<String> = conn.get(key).map_err(|e| AppError::System(format!("Redis get error: {}", e)))?;

        match value {
            Some(v) => {
                let deserialized =
                    serde_json::from_str(&v).map_err(|e| AppError::System(format!("Deserialization error: {}", e)))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        let mut conn = self.get_conn()?;
        let _: () = conn.del(key).map_err(|e| AppError::System(format!("Redis delete error: {}", e)))?;
        Ok(())
    }
}
