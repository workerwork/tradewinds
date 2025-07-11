use serde::{Deserialize, Serialize};

/// 获取系统设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemSettingRequest {
    pub key: String,
}

/// 获取系统设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemSettingResponse {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

/// 设置系统设置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSystemSettingRequest {
    pub key: String,
    pub value: String,
}

/// 设置系统设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSystemSettingResponse {
    pub success: bool,
}
