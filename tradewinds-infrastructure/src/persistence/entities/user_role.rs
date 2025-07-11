use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::role::Entity", from = "Column::RoleId", to = "super::role::Column::Id")]
    Role,
}

impl ActiveModelBehavior for ActiveModel {}
