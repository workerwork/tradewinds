use serde::{Deserialize, Serialize};

/// 查询用户列表查询
///
/// 参数：
/// - page: 页码
/// - page_size: 每页条数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub page: u64,
    pub page_size: u64,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
    pub email: Option<String>,
    pub show_deleted: Option<bool>,
}

impl ListUsersQuery {
    pub fn pagination(&self) -> (u64, u64) {
        let limit = self.page_size;
        let offset = self.page.saturating_sub(1) * self.page_size;
        (limit, offset)
    }
}
