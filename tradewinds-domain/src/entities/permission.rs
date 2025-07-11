use chrono::Utc;
use serde::{Deserialize, Serialize};
use tradewinds_error::AppResult;

use crate::value_objects::permission::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: PermissionId,
    pub name: PermissionName,
    pub code: Option<PermissionCode>,
    pub type_: PermissionType,
    pub parent_id: Option<PermissionId>,
    pub path: Option<PermissionPath>,
    pub component: Option<PermissionComponent>,
    pub icon: Option<PermissionIcon>,
    pub sort: PermissionSort,
    pub status: PermissionStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Permission {
    pub fn create(
        name: PermissionName,
        code: Option<PermissionCode>,
        type_: PermissionType,
        parent_id: Option<PermissionId>,
        path: Option<PermissionPath>,
        component: Option<PermissionComponent>,
        icon: Option<PermissionIcon>,
        sort: PermissionSort,
    ) -> AppResult<Permission> {
        let now = Utc::now().timestamp();
        let id = PermissionId::new_v4();
        let status = PermissionStatus::default();

        let permission = Permission {
            id,
            name,
            code,
            type_,
            parent_id,
            path,
            component,
            icon,
            sort,
            status,
            created_at: now,
            updated_at: now,
        };
        Ok(permission)
    }

    /// 更新权限属性
    ///
    /// parent_id 语义：
    /// - None: 顶级权限（无父）
    /// - Some(x): 指定父权限
    pub fn update_profile(
        &mut self,
        name: Option<PermissionName>,
        code: Option<PermissionCode>,
        type_: Option<PermissionType>,
        parent_id: Option<PermissionId>,
        path: Option<PermissionPath>,
        component: Option<PermissionComponent>,
        icon: Option<PermissionIcon>,
        sort: Option<PermissionSort>,
        status: Option<PermissionStatus>,
    ) -> AppResult<()> {
        self.name = name.unwrap_or_else(|| self.name.clone());
        self.code = code.or_else(|| self.code.clone());
        self.type_ = type_.unwrap_or(self.type_);
        self.parent_id = parent_id;
        self.path = path.or_else(|| self.path.clone());
        self.component = component.or_else(|| self.component.clone());
        self.icon = icon.or_else(|| self.icon.clone());
        self.sort = sort.unwrap_or(self.sort);
        self.status = status.unwrap_or(self.status);
        self.updated_at = Utc::now().timestamp();
        Ok(())
    }

    pub fn set_name(&mut self, name: PermissionName) {
        self.name = name;
    }

    pub fn set_code(&mut self, code: PermissionCode) {
        self.code = Some(code);
    }

    pub fn set_type(&mut self, type_: PermissionType) {
        self.type_ = type_;
    }

    pub fn set_parent_id(&mut self, parent_id: PermissionId) {
        self.parent_id = Some(parent_id);
    }

    pub fn set_path(&mut self, path: PermissionPath) {
        self.path = Some(path);
    }

    pub fn set_component(&mut self, component: PermissionComponent) {
        self.component = Some(component);
    }

    pub fn set_icon(&mut self, icon: PermissionIcon) {
        self.icon = Some(icon);
    }

    pub fn set_sort(&mut self, sort: PermissionSort) {
        self.sort = sort;
    }

    pub fn set_status(&mut self, status: PermissionStatus) {
        self.status = status;
    }
}
