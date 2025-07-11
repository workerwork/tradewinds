use crate::entities::permission::Permission;
use crate::value_objects::permission::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType,
};
use chrono::Utc;
use tradewinds_error::AppResult;

/// 权限聚合
#[derive(Debug, Clone)]
pub struct PermissionAggregate {
    pub permission: Permission,
}

impl PermissionAggregate {
    /// 创建新权限聚合
    pub fn create(
        name: PermissionName,
        code: Option<PermissionCode>,
        type_: PermissionType,
        parent_id: Option<PermissionId>,
        path: Option<PermissionPath>,
        component: Option<PermissionComponent>,
        icon: Option<PermissionIcon>,
        sort: PermissionSort,
    ) -> AppResult<Self> {
        let permission = Permission::create(name, code, type_, parent_id, path, component, icon, sort)?;
        Ok(Self { permission })
    }

    /// 更新权限
    pub fn update(
        &mut self,
        name: Option<PermissionName>,
        code: Option<PermissionCode>,
        type_: Option<PermissionType>,
        parent_id: Option<Option<PermissionId>>,
        path: Option<PermissionPath>,
        component: Option<PermissionComponent>,
        icon: Option<PermissionIcon>,
        sort: Option<PermissionSort>,
        status: Option<PermissionStatus>,
    ) -> AppResult<()> {
        let parent_id_to_pass = match parent_id {
            None => self.permission.parent_id.clone(), // 不修改
            Some(None) => None,                        // 清空
            Some(Some(ref pid)) => Some(pid.clone()),  // 设置为指定父权限
        };
        self.permission.update_profile(name, code, type_, parent_id_to_pass, path, component, icon, sort, status)?;
        Ok(())
    }

    /// 删除权限
    pub fn delete(&mut self) -> AppResult<()> {
        if self.permission.status.is_deleted() {
            return Err(tradewinds_error::AppError::Validation("Permission already deleted".into()));
        }
        self.permission.set_status(PermissionStatus::Deleted);
        self.touch();
        Ok(())
    }

    /// 内部更新时间戳
    fn touch(&mut self) {
        self.permission.updated_at = Utc::now().timestamp();
    }
}
