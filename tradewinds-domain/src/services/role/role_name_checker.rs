use crate::domain::repositories::RoleRepository;
use tradewinds_error::AppResult;

pub struct RoleNameChecker<R: RoleRepository> {
    role_repository: R,
}

impl<R: RoleRepository> RoleNameChecker<R> {
    pub fn new(role_repository: R) -> Self {
        Self { role_repository }
    }

    pub async fn is_unique(&self, name: &str) -> AppResult<bool> {
        Ok(self.role_repository.find_by_name(name).await?.is_none())
    }
}
