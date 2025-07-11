use crate::value_objects::{
    permission::permission_id::PermissionId, role::role_id::RoleId,
    role_permission::role_permission_id::RolePermissionId,
};
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RolePermission {
    pub id: RolePermissionId,
    pub role_id: RoleId,
    pub permission_id: PermissionId,
    pub created_at: i64,
    pub updated_at: i64,
}

impl RolePermission {
    pub fn new(role_id: RoleId, permission_id: PermissionId) -> Self {
        let now = Utc::now().timestamp();
        Self { id: RolePermissionId::new_v4(), role_id, permission_id, created_at: now, updated_at: now }
    }

    pub fn role_id(&self) -> &RoleId {
        &self.role_id
    }

    pub fn permission_id(&self) -> &PermissionId {
        &self.permission_id
    }
}
