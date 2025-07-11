use crate::domain::value_objects::{PermissionComponent, PermissionPath};
use tradewinds_error::AppResult;

pub struct PermissionValidator;

impl PermissionValidator {
    pub fn validate_path(path: &Option<PermissionPath>) -> AppResult<()> {
        if let Some(p) = path {
            if !p.value().starts_with('/') {
                return Err(crate::shared::types::AppError::Validation(
                    "Permission path must start with '/'".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn validate_component(component: &Option<PermissionComponent>) -> AppResult<()> {
        if let Some(c) = component {
            if !c.value().contains("/") {
                return Err(crate::shared::types::AppError::Validation(
                    "Component must include folder structure like 'views/xxx.vue'".to_string(),
                ));
            }
        }
        Ok(())
    }
}
