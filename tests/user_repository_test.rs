use chrono::Utc;
use sea_orm::{Database, DatabaseConnection};
use uuid::Uuid;

use tradewinds::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    value_objects::{
        Email,
        auth::{Password, Username},
        user::{Avatar, Phone, RealName, UserId, UserStatus},
    },
};
use tradewinds::infrastructure::persistence::user_repository::SeaOrmUserRepository;

fn build_test_user(id: &str) -> User {
    User {
        id: UserId::new(id.to_string()),
        username: Username::new("testuser".into()).unwrap(),
        email: Email::new("test@example.com".into()).unwrap(),
        password: Password::new("hashed_password".into()).unwrap(),
        real_name: Some(RealName::new("Test Name".into()).unwrap()),
        phone: Some(Phone::new("12345678901".into()).unwrap()),
        avatar: Some(Avatar::new("https://example.com/avatar.png".into()).unwrap()),
        status: UserStatus::new(1).unwrap(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

async fn setup_db() -> DatabaseConnection {
    // 使用 SQLite 内存数据库
    let db = Database::connect("sqlite::memory:").await.expect("Failed to connect to in-memory db");

    // ⚠️ 若你没使用 migration，需要手动创建 user 表
    let schema_sql = r#"
        CREATE TABLE user (
            id TEXT PRIMARY KEY NOT NULL,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            real_name TEXT,
            phone TEXT,
            avatar TEXT,
            status INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
    "#;
    db.execute(sea_orm::Statement::from_string(db.get_database_backend(), schema_sql.to_string()))
        .await
        .expect("Failed to create schema");

    db
}

#[tokio::test]
async fn test_user_repository_create_and_find() {
    let db = setup_db().await;
    let repo = SeaOrmUserRepository::new(db);

    let id = Uuid::new_v4().to_string();
    let user = build_test_user(&id);

    // create
    repo.create(&user).await.expect("Failed to create user");

    // find_by_id
    let fetched = repo.find_by_id(&id).await.expect("Failed to fetch user");
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();

    assert_eq!(fetched.username.value(), "testuser");
    assert_eq!(fetched.email.value(), "test@example.com");
}
