use serde::{Deserialize, Deserializer, Serialize};

use tradewinds_domain::entities::role::Role;
use tradewinds_domain::entities::user::User;
#[rustfmt::skip]
use crate::api::dtos::{
    permission_dto::PermissionResponse, 
    role_dto::RoleResponse
};
use tradewinds_application::queries::auth::user_info::UserInfo;
use tradewinds_common::utils::empty_string_as_none;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "roleIds")]
    pub role_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: String,
    #[serde(rename = "realName", default, deserialize_with = "empty_string_as_none")]
    pub real_name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub avatar: Option<String>,
    pub status: Option<i32>,
    #[serde(rename = "roleIds")]
    pub role_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCurrentUserRequest {
    #[serde(rename = "realName", default, deserialize_with = "empty_string_as_none")]
    pub real_name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRoleResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeRoleRequest {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeRoleResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersRequest {
    pub page: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
    pub email: Option<String>,
    #[serde(alias = "showDeleted")]
    pub show_deleted: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserWithRolesResponse>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByIdResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByUsernameRequest {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByUsernameResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByEmailResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRolesRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRolesResponse {
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserPermissionsRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserPermissionsResponse {
    pub permissions: Vec<PermissionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.to_string(),
            email: user.email.to_string(),
            real_name: user.real_name.map(|v| v.to_string()),
            phone: user.phone.map(|v| v.to_string()),
            avatar: user.avatar.map(|v| v.to_string()),
            status: user.status.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<UserInfo> for UserResponse {
    fn from(info: UserInfo) -> Self {
        UserResponse {
            id: info.id,
            username: info.username,
            email: info.email,
            real_name: info.real_name,
            phone: info.phone,
            avatar: info.avatar,
            status: info.status,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserRolesResponse {
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Serialize)]
pub struct UserPermissionsResponse {
    pub permissions: Vec<PermissionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithRolesResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: String,
    pub roles: Vec<RoleResponse>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserWithRolesResponse {
    pub fn new(user: User, roles: Vec<Role>) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.to_string(),
            email: user.email.to_string(),
            real_name: user.real_name.map(|v| v.to_string()),
            phone: user.phone.map(|v| v.to_string()),
            avatar: user.avatar.map(|v| v.to_string()),
            status: user.status.to_string(),
            roles: roles.into_iter().map(|role| role.into()).collect(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
