use crate::persistence::repositories::SeaOrmTokenBlacklistRepository;
use sea_orm::DatabaseConnection;

pub fn init_token_blacklist_repo(db: &DatabaseConnection) -> SeaOrmTokenBlacklistRepository {
    SeaOrmTokenBlacklistRepository::new(db.clone())
}
