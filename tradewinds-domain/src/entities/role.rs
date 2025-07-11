use crate::value_objects::role::RoleCode;
use crate::value_objects::{RoleDescription, RoleId, RoleName, RoleStatus};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tradewinds_error::AppResult;

// 角色实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: RoleId,
    pub code: RoleCode, // 新增唯一标识字段
    pub name: RoleName,
    pub description: Option<RoleDescription>,
    pub status: RoleStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Role {
    pub fn create(
        id: RoleId,
        code: RoleCode,
        name: RoleName,
        description: Option<RoleDescription>,
        status: RoleStatus,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Role { id, code, name, description, status, created_at, updated_at }
    }

    pub fn update_profile(
        &mut self,
        name: Option<RoleName>,
        description: Option<RoleDescription>,
        status: Option<RoleStatus>,
    ) -> AppResult<()> {
        self.name = name.unwrap_or_else(|| self.name.clone());
        self.description = description.or_else(|| self.description.clone());
        self.status = status.unwrap_or(self.status);
        self.updated_at = Utc::now().timestamp();
        Ok(())
    }

    pub fn set_name(&mut self, name: RoleName) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: RoleDescription) {
        self.description = Some(description);
    }

    pub fn set_status(&mut self, status: RoleStatus) {
        self.status = status;
    }

    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }
}
