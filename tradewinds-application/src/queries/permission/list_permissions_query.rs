use serde::{Deserialize, Serialize};
use tradewinds_domain::value_objects::permission::{PermissionCode, PermissionName, PermissionStatus, PermissionType};

/// 查询权限列表查询
///
/// 参数：
/// - page: 页码
/// - page_size: 每页条数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsQuery {
    pub page: u64,
    pub page_size: u64,
    pub name: Option<PermissionName>,
    pub code: Option<PermissionCode>,
    pub permission_type: Option<PermissionType>,
    pub status: Option<PermissionStatus>,
    pub show_deleted: Option<bool>,
}

impl ListPermissionsQuery {
    pub fn pagination(&self) -> (u64, u64) {
        let offset = self.page.saturating_sub(1) * self.page_size;
        (self.page_size, offset)
    }
}
