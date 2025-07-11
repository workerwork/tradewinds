// Test comment
use crate::entities::role::Role;
use crate::value_objects::{
    permission::PermissionId,
    role::{RoleCode, RoleDescription, RoleId, RoleName, RoleStatus},
};
use chrono::Utc;
use tradewinds_error::{AppError, AppResult};

/// 角色聚合
#[derive(Debug, Clone)]
pub struct RoleAggregate {
    pub role: Role,
    pub permissions: Vec<PermissionId>,
}

impl RoleAggregate {
    /// 创建新角色聚合
    pub fn create(
        name: RoleName,
        code: RoleCode,
        description: Option<RoleDescription>,
        permissions: Option<Vec<PermissionId>>,
        status: RoleStatus,
    ) -> AppResult<Self> {
        let now = Utc::now().timestamp();
        let id = RoleId::new_v4();
        let role = Role::create(id, code, name, description, status, now, now);
        Ok(Self { role, permissions: permissions.unwrap_or_default() })
    }

    /// 从已有数据重建角色聚合（用于从数据库加载）
    pub fn from_existing(role: Role, permissions: Vec<PermissionId>) -> Self {
        Self { role, permissions }
    }

    /// 更新角色
    pub fn update(
        &mut self,
        name: Option<RoleName>,
        description: Option<RoleDescription>,
        status: Option<RoleStatus>,
        permissions: Option<Vec<PermissionId>>,
    ) -> AppResult<()> {
        self.role.update_profile(name, description, status)?;
        if let Some(perms) = permissions {
            self.permissions = perms;
            self.touch();
        }
        Ok(())
    }

    /// 删除角色
    pub fn delete(&mut self) -> AppResult<()> {
        if self.role.status == RoleStatus::Deleted {
            return Err(AppError::Validation("Role already deleted".into()));
        }
        self.role.set_status(RoleStatus::Deleted);
        self.touch();

        Ok(())
    }

    /// 分配权限（避免重复）
    pub fn assign_permission(&mut self, permission_id: &PermissionId) -> AppResult<()> {
        if !self.permissions.contains(permission_id) {
            self.permissions.push(permission_id.clone());
            self.touch();
        }
        Ok(())
    }

    /// 移除权限
    pub fn revoke_permission(&mut self, permission_id: &PermissionId) {
        self.permissions.retain(|p| p != permission_id);
        self.touch();
    }

    /// 内部更新时间戳
    fn touch(&mut self) {
        self.role.updated_at = Utc::now().timestamp();
    }
}
