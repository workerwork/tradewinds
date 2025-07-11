use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 6, max = 50))]
    pub password: String,
}
