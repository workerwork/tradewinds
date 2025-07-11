use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "token_blacklist")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub jti: String,
    #[sea_orm(column_type = "String(StringLen::N(36))")]
    pub user_id: String,
    pub expires_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
