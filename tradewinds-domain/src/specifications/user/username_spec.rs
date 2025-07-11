use crate::domain::entities::user::User;
use crate::domain::specifications::specification::Specification;

pub struct UsernameSpecification {
    min_length: usize,
    max_length: usize,
}

impl UsernameSpecification {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        Self { min_length, max_length }
    }
}

impl Specification<User> for UsernameSpecification {
    fn is_satisfied_by(&self, user: &User) -> bool {
        let len = user.username.value().len();
        len >= self.min_length && len <= self.max_length
    }
}