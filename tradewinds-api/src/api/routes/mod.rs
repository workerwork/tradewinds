// 基础能力路由模块
pub mod auth_routes; // 认证与登录
pub mod permission_routes; // 权限管理
pub mod role_routes; // 角色管理
pub mod system_setting_routes; // 系统设置
pub mod user_routes; // 用户管理

// 业务能力路由模块（如有业务模块可在此添加）
// pub mod order_routes;       // 订单管理
// pub mod product_routes;     // 商品管理
// pub mod report_routes;      // 报表管理

// 统一导出基础能力路由
pub use auth_routes::*;
pub use permission_routes::*;
pub use role_routes::*;
pub use system_setting_routes::*;
pub use user_routes::*;

// 统一导出业务能力路由（如有业务模块可在此添加）
// pub use order_routes::*;
// pub use product_routes::*;
// pub use report_routes::*;
