use std::str::FromStr;

use crate::api::dtos::{
    AssignPermissionRequest, CreateRoleRequest, DeleteRoleRequest, GetRoleByIdRequest, GetRoleByNameRequest,
    GetRolePermissionsRequest, ListRolesRequest, RevokePermissionRequest, UpdateRoleRequest,
};
use tradewinds_application::commands::role::{
    AssignPermissionCommand, CreateRoleCommand, DeleteRoleCommand, RevokePermissionCommand, UpdateRoleCommand,
};
use tradewinds_application::queries::role::{
    GetRoleByIdQuery, GetRoleByNameQuery, GetRolePermissionsQuery, ListRolesQuery,
};
use tradewinds_domain::value_objects::role::RoleCode;
use tradewinds_domain::value_objects::{PermissionId, RoleDescription, RoleId, RoleName, RoleStatus, UserId};
use tradewinds_error::AppResult;

pub fn to_create_role_command(req: CreateRoleRequest) -> AppResult<CreateRoleCommand> {
    Ok(CreateRoleCommand {
        name: RoleName::new(req.name)?,
        code: RoleCode::new(req.code)?,
        description: req.description.map(RoleDescription::new).transpose()?,
        permissions: req
            .permission_ids
            .unwrap_or_default()
            .into_iter()
            .map(PermissionId::new)
            .collect::<AppResult<_>>()?,
        status: req.status.map(RoleStatus::from_i32).transpose()?, // 修正类型转换
    })
}

pub fn to_update_role_command(actor_id: String, req: UpdateRoleRequest) -> AppResult<UpdateRoleCommand> {
    Ok(UpdateRoleCommand {
        id: RoleId::new(req.id)?,
        name: req.name.map(RoleName::new).transpose()?,
        description: req.description.map(RoleDescription::new).transpose()?,
        status: req.status.map(RoleStatus::from_i32).transpose()?,
        updated_by: Some(UserId::from_str(&actor_id)?),
        permissions: req
            .permission_ids
            .map(|ids| ids.into_iter().map(PermissionId::new).collect::<AppResult<_>>())
            .transpose()?,
    })
}

pub fn to_delete_role_command(actor_id: String, req: DeleteRoleRequest) -> AppResult<DeleteRoleCommand> {
    Ok(DeleteRoleCommand { id: RoleId::new(req.id)?, deleted_by: Some(UserId::from_str(&actor_id)?) })
}

pub fn to_assign_permission_command(
    actor_id: String,
    req: AssignPermissionRequest,
) -> AppResult<AssignPermissionCommand> {
    Ok(AssignPermissionCommand {
        role_id: RoleId::new(req.role_id)?,
        permission_ids: req
            .permission_ids
            .into_iter()
            .map(PermissionId::new)
            .collect::<AppResult<Vec<PermissionId>>>()?,
        assigned_by: UserId::from_str(&actor_id)?,
    })
}

pub fn to_revoke_permission_command(
    actor_id: String,
    req: RevokePermissionRequest,
) -> AppResult<RevokePermissionCommand> {
    Ok(RevokePermissionCommand {
        role_id: RoleId::new(req.role_id)?,
        permission_id: PermissionId::new(req.permission_id)?,
        revoked_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_get_role_by_id_query(req: GetRoleByIdRequest) -> AppResult<GetRoleByIdQuery> {
    Ok(GetRoleByIdQuery { role_id: RoleId::new(req.id)? })
}

pub fn to_get_role_by_name_query(req: GetRoleByNameRequest) -> AppResult<GetRoleByNameQuery> {
    Ok(GetRoleByNameQuery { role_name: RoleName::new(req.name)? })
}

pub fn to_get_role_permissions_query(req: GetRolePermissionsRequest) -> AppResult<GetRolePermissionsQuery> {
    Ok(GetRolePermissionsQuery { role_id: RoleId::new(req.role_id)? })
}

pub fn to_list_roles_query(req: ListRolesRequest) -> AppResult<ListRolesQuery> {
    Ok(ListRolesQuery {
        page: req.page,
        page_size: req.page_size,
        name: req.name.clone(),
        code: req.code.clone(),
        status: req.status,
        show_deleted: req.show_deleted,
    })
}
