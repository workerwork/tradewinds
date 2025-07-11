use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QuerySelect, Set,
};

use crate::persistence::{
    entities::{role, user, user_role},
    repositories::sea_orm_user_repository::SeaOrmUserRepository,
};
use tradewinds_domain::entities::{User, UserRole};
use tradewinds_domain::repositories::UserRoleRepository;
use tradewinds_domain::value_objects::{RoleId, UserId, UserRoleId};
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct SeaOrmUserRoleRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn from_model(&self, model: user_role::Model) -> AppResult<UserRole> {
        Ok(UserRole {
            id: UserRoleId::new(model.id)?,
            user_id: UserId::new(model.user_id)?,
            role_id: RoleId::new(model.role_id)?,
            created_at: model.created_at.timestamp(),
            updated_at: model.updated_at.timestamp(),
        })
    }

    fn to_active_model(&self, user_role: &UserRole) -> user_role::ActiveModel {
        let now: DateTime<Utc> = Utc::now();
        user_role::ActiveModel {
            id: Set(user_role.id.value().to_string()),
            user_id: Set(user_role.user_id.value().to_string()),
            role_id: Set(user_role.role_id.value().to_string()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        }
    }
}

#[async_trait]
impl UserRoleRepository for SeaOrmUserRoleRepository {
    async fn create(&self, user_role: &UserRole) -> AppResult<()> {
        let model = self.to_active_model(user_role);
        model.insert(&self.db).await.map_err(|e| AppError::DatabaseError(format!("Create user role failed: {}", e)))?;
        Ok(())
    }

    async fn delete(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id.value()))
            .filter(user_role::Column::RoleId.eq(role_id.value()))
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Revoke user role failed: {}", e)))?;
        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<UserRole>> {
        user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id.value()))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Find roles by user id failed: {}", e)))?
            .into_iter()
            .map(|m| self.from_model(m))
            .collect()
    }

    async fn find_users_by_role_id(&self, role_id: &RoleId) -> AppResult<Vec<User>> {
        let role_model = role::Entity::find_by_id(role_id.value())
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let user_models = if let Some(model) = role_model {
            vec![] // TODO: 实现join查询
        } else {
            vec![]
        };

        Ok(user_models)
    }

    async fn exists(&self, user_id: &UserId, role_id: &RoleId) -> AppResult<bool> {
        let count = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id.value()))
            .filter(user_role::Column::RoleId.eq(role_id.value()))
            .count(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Check user role existence failed: {}", e)))?;
        Ok(count > 0)
    }
}
