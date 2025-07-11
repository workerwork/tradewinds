/// 应用层接口
///
/// 认证服务接口: 定义了认证服务的基本操作，包括用户注册、登录、修改密码、登出和获取当前用户。
/// 用户服务接口: 定义了用户服务的基本操作，包括创建、更新、删除、分配角色和撤销角色。
/// 角色服务接口: 定义了角色服务的基本操作，包括创建、更新、删除、分配权限和撤销权限。
/// 权限服务接口: 定义了权限服务的基本操作，包括创建、更新、删除、获取和列出权限。
/// 系统设置服务接口: 定义了系统设置服务的基本操作，包括获取和设置系统设置。
pub mod auth_service;
pub mod permission_service;
pub mod role_service;
pub mod user_service;
pub mod system_setting_service;

pub use auth_service::IAuthService;
pub use permission_service::IPermissionService;
pub use role_service::IRoleService;
pub use user_service::IUserService;
pub use system_setting_service::ISystemSettingService;