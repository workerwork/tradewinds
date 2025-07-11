use serde::{self, Deserialize, Deserializer, Serialize};

use tradewinds_application::queries::auth::user_info::PermissionInfo;
use tradewinds_domain::entities::permission::Permission;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub permission_type: String,
    #[serde(rename = "parentId", default, deserialize_with = "null_string_as_none")]
    pub parent_id: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub sort: Option<i32>,
}

// 适用于 Option<String> 的自定义反序列化，兼容 null 和 "null"
fn null_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        Some("null") => Ok(None),
        _ => Ok(opt),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
    pub id: String,
    pub name: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    #[serde(rename = "parentId", default, deserialize_with = "null_string_as_none_nested")]
    pub parent_id: Option<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionResponse {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub permission_type: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Permission> for PermissionResponse {
    fn from(permission: Permission) -> Self {
        Self {
            id: permission.id.to_string(),
            name: permission.name.to_string(),
            code: permission.code.map(|c| c.to_string()),
            permission_type: permission.type_.to_string(),
            parent_id: permission.parent_id.map(|p| p.to_string()),
            path: permission.path.map(|p| p.to_string()),
            component: permission.component.map(|c| c.to_string()),
            icon: permission.icon.map(|i| i.to_string()),
            sort: *permission.sort,
            status: permission.status.to_string(),
            created_at: permission.created_at,
            updated_at: permission.updated_at,
        }
    }
}

impl From<PermissionInfo> for PermissionResponse {
    fn from(info: PermissionInfo) -> Self {
        PermissionResponse {
            id: info.id,
            name: info.name,
            code: info.code,
            permission_type: info.type_,
            parent_id: info.parent_id,
            path: info.path,
            component: info.component,
            icon: info.icon,
            sort: info.sort,
            status: info.status,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionListResponse {
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionResponse {
    pub permission: PermissionResponse,
}

#[derive(Debug, Deserialize)]
pub struct DeletePermissionRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePermissionResponse;

#[derive(Debug, Deserialize)]
pub struct GetPermissionByNameRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPermissionByNameResponse {
    pub permission: PermissionResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetPermissionByCodeRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPermissionByCodeResponse {
    pub permission: PermissionResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetPermissionByIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPermissionByIdResponse {
    pub permission: PermissionResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsRequest {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u64,
    pub name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    pub status: Option<i32>,
    #[serde(rename = "showDeleted")]
    pub show_deleted: Option<bool>,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<PermissionResponse>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListPermissionsByTypeRequest {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsByTypeResponse {
    pub permissions: Vec<PermissionResponse>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListPermissionsByParentIdRequest {
    pub parent_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPermissionsByParentIdResponse {
    pub permissions: Vec<PermissionResponse>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionTreeResponse {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub permission_type: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub children: Vec<PermissionTreeResponse>,
}

impl PermissionTreeResponse {
    pub fn build_tree(flat: Vec<PermissionTreeResponse>) -> Vec<PermissionTreeResponse> {
        let mut map: std::collections::HashMap<String, PermissionTreeResponse> =
            flat.into_iter().map(|node| (node.id.clone(), node)).collect();
        let mut parent_child = Vec::new();
        for id in map.keys().cloned().collect::<Vec<_>>() {
            if let Some(parent_id) = map[&id].parent_id.clone() {
                parent_child.push((parent_id, id));
            }
        }
        for (parent_id, id) in parent_child {
            let node = map.remove(&id);
            if let (Some(parent), Some(node)) = (map.get_mut(&parent_id), node) {
                parent.children.push(node);
            }
        }
        map.into_iter().filter(|(_, node)| node.parent_id.is_none()).map(|(_, node)| node).collect()
    }
}

impl From<Permission> for PermissionTreeResponse {
    fn from(permission: Permission) -> Self {
        PermissionTreeResponse {
            id: permission.id.to_string(),
            name: permission.name.to_string(),
            code: permission.code.map(|c| c.to_string()),
            permission_type: permission.type_.to_string(),
            parent_id: permission.parent_id.map(|p| p.to_string()),
            component: permission.component.map(|c| c.to_string()),
            icon: permission.icon.map(|i| i.to_string()),
            path: permission.path.map(|p| p.to_string()),
            sort: permission.sort.value(),
            status: permission.status.to_string(),
            created_at: permission.created_at,
            updated_at: permission.updated_at,
            children: Vec::new(),
        }
    }
}

// 嵌套Option的自定义反序列化，兼容字符串"null"和null
fn null_string_as_none_nested<'de, D>(deserializer: D) -> Result<Option<Option<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Option<String>>::deserialize(deserializer)?;
    match opt {
        None => Ok(Some(None)), // 兼容 null
        Some(Some(ref s)) if s == "null" => Ok(Some(None)),
        _ => Ok(opt),
    }
}
