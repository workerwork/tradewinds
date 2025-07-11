use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuInfo {
    pub id: String,
    pub name: String,
    pub code: String,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub children: Vec<MenuInfo>,
}
