use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 用户表
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("username")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("email")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("password")).string().not_null())
                    .col(ColumnDef::new(Alias::new("real_name")).string())
                    .col(ColumnDef::new(Alias::new("phone")).string())
                    .col(ColumnDef::new(Alias::new("avatar")).string())
                    .col(ColumnDef::new(Alias::new("status")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 角色表
        manager
            .create_table(
                Table::create()
                    .table("roles")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("name")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("description")).string())
                    .col(ColumnDef::new(Alias::new("status")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 权限表
        manager
            .create_table(
                Table::create()
                    .table("permissions")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("name")).string().not_null())
                    .col(ColumnDef::new(Alias::new("code")).string())
                    .col(ColumnDef::new(Alias::new("type")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("parent_id")).string())
                    .col(ColumnDef::new(Alias::new("path")).string())
                    .col(ColumnDef::new(Alias::new("component")).string())
                    .col(ColumnDef::new(Alias::new("icon")).string())
                    .col(ColumnDef::new(Alias::new("sort")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("status")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // 用户角色关系表
        manager
            .create_table(
                Table::create()
                    .table("user_roles")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("user_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("role_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl("user_roles")
                            .from_col("user_id")
                            .to_tbl("users")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl("user_roles")
                            .from_col("role_id")
                            .to_tbl("roles")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 角色权限关系表
        manager
            .create_table(
                Table::create()
                    .table("role_permissions")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("role_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("permission_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Alias::new("updated_at")).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl("role_permissions")
                            .from_col("role_id")
                            .to_tbl("roles")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl("role_permissions")
                            .from_col("permission_id")
                            .to_tbl("permissions")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // token_blacklist 表
        manager
            .create_table(
                Table::create()
                    .table("token_blacklist")
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).string().not_null().primary_key())
                    .col(ColumnDef::new(Alias::new("jti")).string().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).string().not_null())
                    .col(ColumnDef::new(Alias::new("expires_at")).timestamp_with_time_zone().not_null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl("token_blacklist")
                            .from_col("user_id")
                            .to_tbl("users")
                            .to_col("id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 插入初始数据
        manager.get_connection().execute_unprepared(
            "INSERT INTO users (id, username, email, password, status, created_at, updated_at) VALUES ('1', 'admin', 'admin@example.com', '$2b$12$EoK4mz/Yeb4lrTaJqo9Rou7yIdsqYqrcpnKdXyqvXXheWlP8WN4Uq', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO users (id, username, email, password, status, created_at, updated_at) VALUES ('2', 'user', 'user@example.com', '$2b$12$MZjCZ2c8JKzJLW6HDsgnieQkQaBZlFeZ/OaAepyqNTmdyAUsqYRqe', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO roles (id, name, status, created_at, updated_at) VALUES ('1', 'admin', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO roles (id, name, status, created_at, updated_at) VALUES ('2', 'user', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO permissions (id, name, type, sort, status, created_at, updated_at) VALUES ('1', 'manage_users', 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO permissions (id, name, type, sort, status, created_at, updated_at) VALUES ('2', 'view_dashboard', 1, 2, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO role_permissions (id, role_id, permission_id, created_at, updated_at) VALUES ('1', '1', '1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO role_permissions (id, role_id, permission_id, created_at, updated_at) VALUES ('2', '2', '2', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO user_roles (id, user_id, role_id, created_at, updated_at) VALUES ('1', '1', '1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        manager.get_connection().execute_unprepared(
            "INSERT INTO user_roles (id, user_id, role_id, created_at, updated_at) VALUES ('2', '2', '2', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("token_blacklist").to_owned()).await?;
        manager.drop_table(Table::drop().table("role_permissions").to_owned()).await?;
        manager.drop_table(Table::drop().table("user_roles").to_owned()).await?;
        manager.drop_table(Table::drop().table("permissions").to_owned()).await?;
        manager.drop_table(Table::drop().table("roles").to_owned()).await?;
        manager.drop_table(Table::drop().table("users").to_owned()).await?;
        Ok(())
    }
}
