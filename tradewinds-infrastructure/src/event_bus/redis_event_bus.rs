// use async_trait::async_trait;
// use redis::{Client, Commands, RedisError};
// use std::sync::Arc;
//
// // use crate::{
// //     application::interfaces::event::IEventBus,
// //     domain::services::event_bus::{Event, EventBus},
// //     shared::types::{AppError, AppResult},
// // };
//
// pub struct RedisEventBus {
//     // client: Client,
// }
//
// impl RedisEventBus {
//     // pub fn new(redis_url: &str) -> AppResult<Self> {
//     //     let client = Client::open(redis_url)
//     //         .map_err(|e| AppError::Initialisation(e.to_string()))?;
//     //     Ok(Self { client })
//     // }
// }
//
// // impl EventBus for RedisEventBus {
// //     async fn publish(&self, event: Arc<dyn Event>) -> AppResult<()> {
// //         let mut conn = self.client.get_connection()
// //             .map_err(|e| AppError::Initialisation(e.to_string()))?;
// //         let payload = serde_json::to_string(event.as_ref())
// //             .map_err(|e| AppError::Serialization(e.to_string()))?;
// //         conn.publish(event.topic(), payload)
// //             .map_err(|e| AppError::Database(e.to_string()))?;
// //         Ok(())
// //     }
// // }
//
// // impl IEventBus for RedisEventBus {
// //     async fn publish(&self, event: Arc<dyn Event>) -> AppResult<()> {
// //         EventBus::publish(self, event).await
// //     }
// // }
