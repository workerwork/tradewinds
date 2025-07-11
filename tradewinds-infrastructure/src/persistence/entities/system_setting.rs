use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "system_settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl sea_orm::RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}
