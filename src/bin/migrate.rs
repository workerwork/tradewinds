use dotenv::dotenv;
use sea_orm::Database;
use sea_orm_migration::prelude::*;
use tradewinds_error::{AppError, AppResult};
use tradewinds_infrastructure::config::AppConfig;
use tradewinds_infrastructure::persistence::migrations::Migrator;

#[tokio::main]
async fn main() -> AppResult<()> {
    // 加载环境变量
    dotenv().ok();

    // 加载配置
    let config = AppConfig::from_env()?;

    // 连接数据库
    let connection = Database::connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(format!("数据库连接失败: {}", e)))?;

    // 执行迁移
    Migrator::up(&connection, None).await.map_err(|e| AppError::DatabaseError(format!("数据库迁移失败: {}", e)))?;

    Ok(())
}
