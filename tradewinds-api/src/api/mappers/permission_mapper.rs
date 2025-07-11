use std::str::FromStr;

use crate::api::dtos::permission_dto::PermissionTreeResponse;
use crate::api::dtos::{
    CreatePermissionRequest, DeletePermissionRequest, GetPermissionByCodeRequest, GetPermissionByIdRequest,
    GetPermissionByNameRequest, ListPermissionsByParentIdRequest, ListPermissionsByTypeRequest, ListPermissionsRequest,
    UpdatePermissionRequest,
};
use tradewinds_application::commands::permission::{
    CreatePermissionCommand, DeletePermissionCommand, UpdatePermissionCommand,
};
use tradewinds_application::queries::permission::{
    GetPermissionByCodeQuery, GetPermissionByIdQuery, GetPermissionByNameQuery, ListPermissionsByParentIdQuery,
    ListPermissionsByTypeQuery, ListPermissionsQuery,
};
use tradewinds_domain::entities::permission::Permission;
use tradewinds_domain::value_objects::{
    PermissionCode, PermissionComponent, PermissionIcon, PermissionId, PermissionName, PermissionPath, PermissionSort,
    PermissionStatus, PermissionType, UserId,
};
use tradewinds_error::AppResult;

pub fn to_create_permission_command(
    actor_id: String,
    req: CreatePermissionRequest,
) -> AppResult<CreatePermissionCommand> {
    Ok(CreatePermissionCommand {
        name: PermissionName::new(req.name)?,
        code: Some(PermissionCode::new(req.code)?),
        type_: PermissionType::from_str(&req.permission_type)?,
        parent_id: req.parent_id.map(PermissionId::new).transpose()?,
        path: req.path.map(PermissionPath::new).transpose()?,
        component: req.component.map(PermissionComponent::new).transpose()?,
        icon: req.icon.map(PermissionIcon::new).transpose()?,
        sort: req.sort.map(PermissionSort::new).transpose()?.unwrap_or_default(),
        created_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_update_permission_command(
    actor_id: String,
    req: UpdatePermissionRequest,
) -> AppResult<UpdatePermissionCommand> {
    Ok(UpdatePermissionCommand {
        id: PermissionId::new(req.id)?,
        name: req.name.map(PermissionName::new).transpose()?,
        path: req.path.map(PermissionPath::new).transpose()?,
        component: req.component.map(PermissionComponent::new).transpose()?,
        icon: req.icon.map(PermissionIcon::new).transpose()?,
        sort: req.sort.map(PermissionSort::new).transpose()?,
        status: req.status.map(PermissionStatus::from_i32).transpose()?,
        code: None,
        type_: req.permission_type.map(|t| PermissionType::from_str(&t)).transpose()?,
        parent_id: match req.parent_id {
            None => None,                                                       // 不修改父权限
            Some(None) => Some(None),                                           // 清空父权限
            Some(Some(ref pid)) => Some(Some(PermissionId::new(pid.clone())?)), // 设置为指定父权限
        },
        updated_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_delete_permission_command(
    actor_id: String,
    req: DeletePermissionRequest,
) -> AppResult<DeletePermissionCommand> {
    Ok(DeletePermissionCommand {
        permission_id: PermissionId::new(req.id)?,
        deleted_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_get_permission_by_id_query(req: GetPermissionByIdRequest) -> AppResult<GetPermissionByIdQuery> {
    Ok(GetPermissionByIdQuery { permission_id: PermissionId::new(req.id)? })
}

pub fn to_get_permission_by_name_query(req: GetPermissionByNameRequest) -> AppResult<GetPermissionByNameQuery> {
    Ok(GetPermissionByNameQuery { permission_name: PermissionName::new(req.name)? })
}

pub fn to_get_permission_by_code_query(req: GetPermissionByCodeRequest) -> AppResult<GetPermissionByCodeQuery> {
    Ok(GetPermissionByCodeQuery { permission_code: PermissionCode::new(req.code)? })
}

pub fn to_list_permissions_query(req: ListPermissionsRequest) -> AppResult<ListPermissionsQuery> {
    Ok(ListPermissionsQuery {
        page: req.page,
        page_size: req.page_size,
        name: req.name.map(PermissionName::new).transpose()?,
        code: req.code.map(PermissionCode::new).transpose()?,
        permission_type: req.permission_type.map(|t| PermissionType::from_str(&t)).transpose()?,
        status: req.status.map(|s| PermissionStatus::from_i32(s)).transpose()?,
        show_deleted: req.show_deleted,
    })
}

pub fn to_list_permissions_by_type_query(req: ListPermissionsByTypeRequest) -> AppResult<ListPermissionsByTypeQuery> {
    Ok(ListPermissionsByTypeQuery { permission_type: PermissionType::from_str(&req.r#type)? })
}

pub fn to_list_permissions_by_parent_id_query(
    req: ListPermissionsByParentIdRequest,
) -> AppResult<ListPermissionsByParentIdQuery> {
    Ok(ListPermissionsByParentIdQuery { parent_id: PermissionId::new(req.parent_id)? })
}

pub fn to_permission_tree_response(
    permission: &Permission,
    children: Vec<PermissionTreeResponse>,
) -> PermissionTreeResponse {
    PermissionTreeResponse {
        id: permission.id.to_string(),
        name: permission.name.to_string(),
        code: permission.code.as_ref().map(|c| c.to_string()),
        permission_type: permission.type_.to_string(),
        parent_id: permission.parent_id.as_ref().map(|p| p.to_string()),
        path: permission.path.as_ref().map(|p| p.to_string()),
        component: permission.component.as_ref().map(|c| c.to_string()),
        icon: permission.icon.as_ref().map(|i| i.to_string()),
        sort: *permission.sort,
        status: permission.status.to_string(),
        created_at: permission.created_at,
        updated_at: permission.updated_at,
        children,
    }
}

pub fn to_tree_responses(flat: Vec<Permission>) -> Vec<PermissionTreeResponse> {
    let flat: Vec<PermissionTreeResponse> = flat.into_iter().map(PermissionTreeResponse::from).collect();
    PermissionTreeResponse::build_tree(flat)
}
