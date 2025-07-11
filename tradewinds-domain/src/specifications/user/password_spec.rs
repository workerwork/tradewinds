use crate::domain::entities::user::User;
use crate::domain::specifications::specification::Specification;

pub struct PasswordSpecification {
    min_length: usize,
    requires_number: bool,
    requires_special_char: bool,
}

impl PasswordSpecification {
    pub fn new(min_length: usize, requires_number: bool, requires_special_char: bool) -> Self {
        Self { min_length, requires_number, requires_special_char }
    }
}

impl Specification<User> for PasswordSpecification {
    fn is_satisfied_by(&self, user: &User) -> bool {
        let password = user.password.value();

        if password.len() < self.min_length {
            return false;
        }

        if self.requires_number && !password.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }

        if self.requires_special_char && !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
            return false;
        }

        true
    }
}