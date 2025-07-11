use serde::{Deserialize, Serialize};

use crate::api::dtos::permission_dto::PermissionResponse;
use tradewinds_application::queries::auth::user_info::RoleInfo;
use tradewinds_domain::entities::{permission::Permission, role::Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    #[serde(rename = "permissionIds")]
    pub permission_ids: Option<Vec<String>>,
    pub status: Option<i32>, // 新增，支持指定角色状态
}

#[derive(Debug, Serialize)]
pub struct CreateRoleResponse {
    pub role: RoleResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
    #[serde(rename = "permissionIds")]
    pub permission_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateRoleResponse;

#[derive(Debug, Deserialize)]
pub struct DeleteRoleRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteRoleResponse;

#[derive(Debug, Deserialize)]
pub struct GetRoleByIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct GetRoleByIdResponse {
    pub role: RoleResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetRoleByNameRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GetRoleByNameResponse {
    pub role: RoleResponse,
}

#[derive(Debug, Deserialize)]
pub struct GetRolePermissionsRequest {
    pub role_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetRolePermissionsResponse {
    pub permissions: Vec<PermissionResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListRolesRequest {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u64,
    pub name: Option<String>,
    pub code: Option<String>,
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

#[derive(Debug, Serialize)]
pub struct ListRolesResponse {
    pub roles: Vec<RoleWithPermissionsResponse>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleResponse {
    pub id: String,
    pub code: String, // 新增唯一标识字段
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        RoleResponse {
            id: role.id.value().to_string(),
            code: role.code.value().to_string(),
            name: role.name.value().to_string(),
            description: role.description.map(|d| d.value().to_string()),
            status: role.status.value(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<RoleInfo> for RoleResponse {
    fn from(info: RoleInfo) -> Self {
        RoleResponse {
            id: info.id,
            code: info.code,
            name: info.name,
            description: info.description,
            status: info.status,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleWithPermissionsResponse {
    pub id: String,
    pub code: String, // 新增唯一标识字段
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub permissions: Vec<PermissionResponse>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl RoleWithPermissionsResponse {
    pub fn new(role: Role, permissions: Vec<Permission>) -> Self {
        Self {
            id: role.id.to_string(),
            code: role.code.to_string(),
            name: role.name.to_string(),
            description: role.description.map(|d| d.to_string()),
            status: role.status.value(),
            permissions: permissions.into_iter().map(|permission| permission.into()).collect(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoleListResponse {
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    pub role_id: String,
    pub permission_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AssignPermissionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokePermissionRequest {
    pub role_id: String,
    pub permission_id: String,
}

#[derive(Debug, Serialize)]
pub struct RevokePermissionResponse;

#[derive(Debug, Serialize)]
pub struct RolePermissionsResponse {
    pub permissions: Vec<PermissionResponse>,
}
