use serde::{Deserialize, Serialize};

/// 查询角色列表查询
///
/// 参数：
/// - page: 页码
/// - page_size: 每页条数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesQuery {
    pub page: u64,
    pub page_size: u64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub status: Option<i32>,
    pub show_deleted: Option<bool>,
}

impl ListRolesQuery {
    pub fn pagination(&self) -> (u64, u64) {
        let offset = self.page.saturating_sub(1) * self.page_size;
        (self.page_size, offset)
    }
}
