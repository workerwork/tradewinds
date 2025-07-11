use crate::domain::entities::{Role, Permission};

pub struct RolePermissionSyncer;

impl RolePermissionSyncer {
    pub fn sync_permissions(role: &mut Role, permissions: Vec<Permission>) {
        role.set_permissions(permissions);
    }
}