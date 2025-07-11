use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    user::UserId,
    permission::{
        PermissionCode,
        PermissionComponent,
        PermissionIcon,
        PermissionId,
        PermissionName,
        PermissionPath,
        PermissionSort,
        PermissionType,
    },
};

/// 创建权限命令
///
/// 参数：
/// - name: 权限名称
/// - code: 权限代码
/// - type_: 权限类型
/// - parent_id: 父权限ID
/// - path: 权限路径
/// - component: 权限组件
/// - icon: 权限图标
/// - sort: 权限排序
/// - created_by: 创建者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionCommand {
    pub name: PermissionName,
    pub code: Option<PermissionCode>,
    pub type_: PermissionType,
    pub parent_id: Option<PermissionId>,
    pub path: Option<PermissionPath>,
    pub component: Option<PermissionComponent>,
    pub icon: Option<PermissionIcon>,
    pub sort: PermissionSort,
    pub created_by: Option<UserId>,
}
