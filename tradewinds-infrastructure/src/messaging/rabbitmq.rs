use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{
    BasicProperties, Connection, ConnectionProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
};
use serde::{Serialize, de::DeserializeOwned};
use tradewinds_error::{AppError, AppResult};

#[async_trait]
pub trait MessageQueue: Send + Sync {
    async fn publish<T: Serialize + Send + Sync>(&self, queue: &str, message: &T) -> AppResult<()>;
    async fn subscribe<T: DeserializeOwned + Send + 'static>(
        &self,
        queue: &str,
        callback: impl Fn(T) -> AppResult<()> + Send + Sync + 'static,
    ) -> AppResult<()>;
}

pub struct RabbitMQ {
    connection: Connection,
}

impl RabbitMQ {
    pub async fn new(amqp_url: &str) -> AppResult<Self> {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default())
            .await
            .map_err(|e| AppError::System(format!("RabbitMQ connection error: {}", e)))?;

        Ok(Self { connection })
    }
}

#[async_trait]
impl MessageQueue for RabbitMQ {
    async fn publish<T: Serialize + Send + Sync>(&self, queue: &str, message: &T) -> AppResult<()> {
        let channel = self
            .connection
            .create_channel()
            .await
            .map_err(|e| AppError::System(format!("Channel creation error: {}", e)))?;

        channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| AppError::System(format!("Queue declare error: {}", e)))?;

        let payload =
            serde_json::to_vec(message).map_err(|e| AppError::System(format!("Message serialization error: {}", e)))?;

        channel
            .basic_publish("", queue, BasicPublishOptions::default(), &payload, BasicProperties::default())
            .await
            .map_err(|e| AppError::System(format!("Message publish error: {}", e)))?;

        Ok(())
    }

    async fn subscribe<T: DeserializeOwned + Send + 'static>(
        &self,
        queue: &str,
        callback: impl Fn(T) -> AppResult<()> + Send + Sync + 'static,
    ) -> AppResult<()> {
        let channel = self
            .connection
            .create_channel()
            .await
            .map_err(|e| AppError::System(format!("Channel creation error: {}", e)))?;

        channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| AppError::System(format!("Queue declare error: {}", e)))?;

        let mut consumer = channel
            .basic_consume(queue, "", lapin::options::BasicConsumeOptions::default(), FieldTable::default())
            .await
            .map_err(|e| AppError::System(format!("Consumer creation error: {}", e)))?;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                if let Ok(message) = serde_json::from_slice::<T>(&delivery.data) {
                    if let Err(e) = callback(message) {
                        tracing::error!("Message processing error: {}", e);
                    }
                }
                delivery
                    .ack(lapin::options::BasicAckOptions::default())
                    .await
                    .map_err(|e| AppError::System(format!("Message acknowledgement error: {}", e)))?;
            }
        }

        Ok(())
    }
}
