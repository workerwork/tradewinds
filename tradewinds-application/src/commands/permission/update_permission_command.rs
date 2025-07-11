use serde::{Deserialize, Serialize};

#[rustfmt::skip]
use tradewinds_domain::value_objects::{
    permission::{
        PermissionId, 
        PermissionName,
        PermissionCode, 
        PermissionComponent, 
        PermissionIcon, 
        PermissionPath,
        PermissionSort,
        PermissionStatus,
        PermissionType,
    },
    user::UserId,
};

/// 更新权限命令
///
/// 参数：
/// - id: 权限ID
/// - name: 权限名称
/// - code: 权限代码
/// - type_: 权限类型
/// - parent_id: 父权限ID
/// - path: 权限路径
/// - component: 权限组件
/// - icon: 权限图标
/// - sort: 权限排序
/// - status: 权限状态
/// - updated_by: 更新者ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionCommand {
    pub id: PermissionId,
    pub name: Option<PermissionName>,
    pub code: Option<PermissionCode>,
    pub type_: Option<PermissionType>,
    pub parent_id: Option<Option<PermissionId>>,
    pub path: Option<PermissionPath>,
    pub component: Option<PermissionComponent>,
    pub icon: Option<PermissionIcon>,
    pub sort: Option<PermissionSort>,
    pub status: Option<PermissionStatus>,
    pub updated_by: Option<UserId>,
}
