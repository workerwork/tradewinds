use crate::domain::entities::user::User;
use crate::domain::specifications::specification::Specification;

pub struct ActiveUserSpecification;

impl Specification<User> for ActiveUserSpecification {
    fn is_satisfied_by(&self, user: &User) -> bool {
        user.status.is_active()
    }
}