pub mod commands;
pub mod events;
pub mod interfaces;
pub mod queries;
pub mod services;

use tradewinds_error::AppResult;

#[async_trait::async_trait]
pub trait CommandHandler<C, O>: Send + Sync {
    async fn handle(&self, command: C) -> AppResult<O>;
}

#[async_trait::async_trait]
pub trait QueryHandler<Q, O>: Send + Sync {
    async fn handle(&self, query: Q) -> AppResult<O>;
}
