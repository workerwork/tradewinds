use serde::{Deserialize, Serialize};

/// 查询全部权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAllPermissionsQuery;
