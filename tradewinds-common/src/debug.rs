use std::fmt::Debug;
use tracing::{debug, error, info, warn};

/// 调试工具结构体
pub struct DebugHelper;

impl DebugHelper {
    /// 记录函数入口
    pub fn enter_function(function_name: &str, args: Option<&str>) {
        match args {
            Some(args) => debug!("🔵 进入函数: {} - 参数: {}", function_name, args),
            None => debug!("🔵 进入函数: {}", function_name),
        }
    }

    /// 记录函数退出
    pub fn exit_function(function_name: &str, result: Option<&str>) {
        match result {
            Some(result) => debug!("🟢 退出函数: {} - 结果: {}", function_name, result),
            None => debug!("🟢 退出函数: {}", function_name),
        }
    }

    /// 记录数据库操作
    pub fn db_operation(operation: &str, table: &str, details: Option<&str>) {
        match details {
            Some(details) => info!("🗄️ 数据库操作: {} on {} - {}", operation, table, details),
            None => info!("🗄️ 数据库操作: {} on {}", operation, table),
        }
    }

    /// 记录错误详情
    pub fn log_error<T: Debug>(error: &T, context: &str, location: Option<&str>) {
        match location {
            Some(location) => error!("❌ 错误 [{}]: {} - 详情: {:?}", location, context, error),
            None => error!("❌ 错误: {} - 详情: {:?}", context, error),
        }
    }

    /// 记录警告
    pub fn log_warning(message: &str, context: Option<&str>) {
        match context {
            Some(context) => warn!("⚠️ 警告 [{}]: {}", context, message),
            None => warn!("⚠️ 警告: {}", message),
        }
    }

    /// 记录业务逻辑执行
    pub fn business_logic(operation: &str, entity: &str, details: Option<&str>) {
        match details {
            Some(details) => info!("💼 业务逻辑: {} {} - {}", operation, entity, details),
            None => info!("💼 业务逻辑: {} {}", operation, entity),
        }
    }

    /// 记录性能信息
    pub fn performance(operation: &str, duration_ms: u128) {
        if duration_ms > 1000 {
            warn!("🐌 性能警告: {} 耗时 {}ms", operation, duration_ms);
        } else {
            debug!("⚡ 性能: {} 耗时 {}ms", operation, duration_ms);
        }
    }
}

/// 便捷宏：记录函数执行
#[macro_export]
macro_rules! debug_fn {
    ($fn_name:expr) => {
        tradewinds_common::debug::DebugHelper::enter_function($fn_name, None);
    };
    ($fn_name:expr, $args:expr) => {
        tradewinds_common::debug::DebugHelper::enter_function($fn_name, Some($args));
    };
}

/// 便捷宏：记录函数退出
#[macro_export]
macro_rules! debug_fn_exit {
    ($fn_name:expr) => {
        tradewinds_common::debug::DebugHelper::exit_function($fn_name, None);
    };
    ($fn_name:expr, $result:expr) => {
        tradewinds_common::debug::DebugHelper::exit_function($fn_name, Some($result));
    };
}

/// 便捷宏：记录错误
#[macro_export]
macro_rules! debug_error {
    ($error:expr, $context:expr) => {
        tradewinds_common::debug::DebugHelper::log_error(&$error, $context, None);
    };
    ($error:expr, $context:expr, $location:expr) => {
        tradewinds_common::debug::DebugHelper::log_error(&$error, $context, Some($location));
    };
}

/// 便捷宏：性能监控
#[macro_export]
macro_rules! time_it {
    ($operation:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed().as_millis();
        tradewinds_common::debug::DebugHelper::performance($operation, duration);
        result
    }};
}
