use serde::{Deserialize, Serialize};
use tradewinds_domain::entities::{Permission, Role, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUserInfo {
    // 用户基础信息
    pub user: UserInfo,
    // 角色信息
    pub roles: Vec<RoleInfo>,
    // 权限信息
    pub permissions: Vec<PermissionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    pub type_: String,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: i32,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
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

impl From<Role> for RoleInfo {
    fn from(role: Role) -> Self {
        RoleInfo {
            id: role.id.to_string(),
            code: role.code.to_string(),
            name: role.name.to_string(),
            description: role.description.map(|v| v.to_string()),
            status: role.status.value(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<Permission> for PermissionInfo {
    fn from(p: Permission) -> Self {
        PermissionInfo {
            id: p.id.to_string(),
            name: p.name.to_string(),
            code: p.code.map(|v| v.to_string()),
            type_: p.type_.to_string(),
            parent_id: p.parent_id.map(|v| v.to_string()),
            path: p.path.map(|v| v.to_string()),
            component: p.component.map(|v| v.to_string()),
            icon: p.icon.map(|v| v.to_string()),
            sort: p.sort.value(),
            status: p.status.to_string(),
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
