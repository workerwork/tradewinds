use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub type_: i32, // 0=menu, 1=button, 2=api
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: i32,
    pub status: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::role_permission::Entity",
        from = "Column::Id",
        to = "super::role_permission::Column::PermissionId"
    )]
    RolePermission,
}

impl ActiveModelBehavior for ActiveModel {}
