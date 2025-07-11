use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginValidator {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(length(min = 6, max = 100))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterValidator {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 100))]
    pub password: String,

    #[validate(must_match = "password")]
    pub password_confirmation: String,
}
