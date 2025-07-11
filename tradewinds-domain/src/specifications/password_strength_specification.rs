use crate::specifications::Specification;

pub struct PasswordStrengthSpecification {
    min_length: usize,
    requires_uppercase: bool,
    requires_lowercase: bool,
    requires_number: bool,
    requires_special_char: bool,
}

impl PasswordStrengthSpecification {
    pub fn new(
        min_length: usize,
        requires_uppercase: bool,
        requires_lowercase: bool,
        requires_number: bool,
        requires_special_char: bool,
    ) -> Self {
        Self { min_length, requires_uppercase, requires_lowercase, requires_number, requires_special_char }
    }
}

impl Specification<str> for PasswordStrengthSpecification {
    fn is_satisfied_by(&self, candidate: &str) -> bool {
        // 具体校验逻辑，比如长度、字符种类等
        candidate.len() >= self.min_length
            && candidate.chars().any(|c| self.requires_uppercase && c.is_uppercase())
            && candidate.chars().any(|c| self.requires_lowercase && c.is_lowercase())
            && candidate.chars().any(|c| self.requires_number && c.is_digit(10))
            && candidate.chars().any(|c| self.requires_special_char && !c.is_ascii_alphanumeric())
    }
    fn message(&self) -> String {
        "Password must be at least 8 characters long and contain at least one uppercase letter".to_string()
    }
}
