/// 主函数入口
/// 1. 加载环境变量
/// 2. 初始化日志
/// 3. 加载配置
/// 4. 创建应用
/// 5. 运行应用
use chrono::{DateTime, TimeZone, Utc};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tradewinds_infrastructure::logging::init_logging();

    let config = tradewinds_infrastructure::config::AppConfig::from_env().expect("Failed to load config");

    let app = tradewinds::app::App::new(config).await.expect("Failed to create app");

    app.run().await.expect("Failed to run app");
}
