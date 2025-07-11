use std::str::FromStr;

use crate::api::dtos::{
    AssignRoleRequest, CreateUserRequest, DeleteUserRequest, GetUserByEmailRequest, GetUserByIdRequest,
    GetUserByUsernameRequest, GetUserPermissionsRequest, GetUserRolesRequest, ListUsersRequest, RevokeRoleRequest,
    UpdateUserRequest,
};
use tradewinds_application::commands::user::{
    AssignRoleCommand, CreateUserCommand, DeleteUserCommand, ResetPasswordCommand, RevokeRoleCommand, UpdateUserCommand,
};
use tradewinds_application::queries::user::{
    GetUserByEmailQuery, GetUserByIdQuery, GetUserByUsernameQuery, GetUserPermissionsQuery, GetUserRolesQuery,
    ListUsersQuery,
};
use tradewinds_domain::value_objects::{
    AuthUsername, Avatar, Email, Password, Phone, RealName, RoleId, UserId, UserStatus,
};

use tradewinds_error::AppResult;

pub fn to_create_user_command(actor_id: String, req: CreateUserRequest) -> AppResult<CreateUserCommand> {
    let role_ids = if let Some(roles) = req.role_ids {
        let mut role_ids = Vec::new();
        for role_id_str in roles {
            role_ids.push(RoleId::new(role_id_str)?);
        }
        Some(role_ids)
    } else {
        None
    };

    Ok(CreateUserCommand {
        username: AuthUsername::new(req.username)?,
        email: Email::new(req.email)?,
        password: Password::new(req.password)?,
        real_name: req.real_name.map(RealName::new).transpose()?,
        phone: req.phone.map(Phone::new).transpose()?,
        avatar: req.avatar.map(Avatar::new).transpose()?,
        role_ids,
        created_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_update_user_command(actor_id: String, req: UpdateUserRequest) -> AppResult<UpdateUserCommand> {
    let role_ids = if let Some(roles) = req.role_ids.clone() {
        let mut role_ids = Vec::new();
        for role_id_str in roles {
            role_ids.push(RoleId::new(role_id_str)?);
        }
        Some(role_ids)
    } else {
        None
    };
    Ok(UpdateUserCommand {
        id: UserId::from_str(&req.id)?,
        real_name: req.real_name.map(RealName::new).transpose()?,
        phone: req.phone.map(Phone::new).transpose()?,
        avatar: req.avatar.map(Avatar::new).transpose()?,
        status: req.status.map(|s| UserStatus::from_i32(s)).transpose()?,
        email: req.email.map(Email::new).transpose()?,
        role_ids,
        updated_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_delete_user_command(actor_id: String, req: DeleteUserRequest) -> AppResult<DeleteUserCommand> {
    Ok(DeleteUserCommand { id: UserId::from_str(&req.id)?, deleted_by: Some(UserId::from_str(&actor_id)?) })
}

pub fn to_assign_role_command(actor_id: String, req: AssignRoleRequest) -> AppResult<AssignRoleCommand> {
    Ok(AssignRoleCommand {
        user_id: UserId::from_str(&req.user_id)?,
        role_id: RoleId::new(req.role_id)?,
        assigned_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_revoke_role_command(actor_id: String, req: RevokeRoleRequest) -> AppResult<RevokeRoleCommand> {
    Ok(RevokeRoleCommand {
        user_id: UserId::from_str(&req.user_id)?,
        role_id: RoleId::new(req.role_id)?,
        revoked_by: Some(UserId::from_str(&actor_id)?),
    })
}

pub fn to_get_user_by_id_query(req: GetUserByIdRequest) -> AppResult<GetUserByIdQuery> {
    Ok(GetUserByIdQuery { user_id: UserId::from_str(&req.id)? })
}

pub fn to_get_user_by_username_query(req: GetUserByUsernameRequest) -> AppResult<GetUserByUsernameQuery> {
    Ok(GetUserByUsernameQuery { username: AuthUsername::new(req.username)? })
}

pub fn to_get_user_by_email_query(req: GetUserByEmailRequest) -> AppResult<GetUserByEmailQuery> {
    Ok(GetUserByEmailQuery { user_email: Email::new(req.email)? })
}

pub fn to_get_user_roles_query(req: GetUserRolesRequest) -> AppResult<GetUserRolesQuery> {
    Ok(GetUserRolesQuery { user_id: UserId::from_str(&req.id)? })
}

pub fn to_get_user_permissions_query(req: GetUserPermissionsRequest) -> AppResult<GetUserPermissionsQuery> {
    Ok(GetUserPermissionsQuery { user_id: UserId::from_str(&req.id)? })
}

pub fn to_list_users_query(req: ListUsersRequest) -> AppResult<ListUsersQuery> {
    Ok(ListUsersQuery {
        page: req.page,
        page_size: req.page_size,
        username: req.username,
        phone: req.phone,
        status: req.status,
        email: req.email,
        show_deleted: req.show_deleted,
    })
}

pub fn to_reset_password_command(actor_id: String, id: String) -> AppResult<ResetPasswordCommand> {
    Ok(ResetPasswordCommand {
        id: UserId::from_str(&id)?,
        reset_by: Some(UserId::from_str(&actor_id)?),
    })
}
