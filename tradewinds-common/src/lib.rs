use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, message: None, data: Some(data) }
    }

    pub fn error(message: &str) -> Self {
        Self { success: false, message: Some(message.to_string()), data: None }
    }
}

pub mod debug;
pub mod utils;
pub use utils::get_current_user_token;
