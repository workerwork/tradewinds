use crate::api::dtos::auth_dto::{GetUserMenusRequest, MenuResponse};
use crate::api::dtos::{
    ChangePasswordRequest, CurrentUserInfoResponse, GetCurrentUserRequest, LoginRequest, LogoutRequest,
    PermissionResponse, RegisterRequest, RoleResponse, UserResponse,
};
use tradewinds_application::commands::{ChangePasswordCommand, LoginCommand, LogoutCommand, RegisterCommand};
use tradewinds_application::queries::GetCurrentUserQuery;
use tradewinds_application::queries::auth::GetUserMenusQuery;
use tradewinds_application::queries::auth::menu_info::MenuInfo;
use tradewinds_application::queries::auth::user_info::CurrentUserInfo;
use tradewinds_domain::value_objects::{AuthUsername, Email, Password, Phone, RealName, Token};
use tradewinds_error::AppResult;

pub fn to_register_command(req: RegisterRequest) -> AppResult<RegisterCommand> {
    Ok(RegisterCommand {
        username: AuthUsername::new(req.username)?,
        email: Email::new(req.email)?,
        password: Password::new(req.password)?,
        real_name: req.real_name.map(RealName::new).transpose()?,
        phone: req.phone.map(Phone::new).transpose()?,
    })
}

pub fn to_login_command(req: LoginRequest) -> AppResult<LoginCommand> {
    Ok(LoginCommand { username: AuthUsername::new(req.username)?, password: Password::new(req.password)? })
}

pub fn to_logout_command(req: LogoutRequest) -> AppResult<LogoutCommand> {
    Ok(LogoutCommand { token: Token::new(req.token)? })
}

pub fn to_change_password_command(token: String, req: ChangePasswordRequest) -> AppResult<ChangePasswordCommand> {
    Ok(ChangePasswordCommand {
        token: Token::new(token)?,
        old_password: Password::new(req.old_password)?,
        new_password: Password::new(req.new_password)?,
    })
}

pub fn to_get_current_user_query(req: GetCurrentUserRequest) -> AppResult<GetCurrentUserQuery> {
    Ok(GetCurrentUserQuery { token: Token::new(req.token)? })
}

pub fn to_get_user_menus_query(req: GetUserMenusRequest) -> AppResult<GetUserMenusQuery> {
    let token = Token::new(req.token)?;
    Ok(GetUserMenusQuery { token })
}

pub fn to_current_user_info_response(info: CurrentUserInfo) -> CurrentUserInfoResponse {
    CurrentUserInfoResponse {
        user: UserResponse::from(info.user),
        roles: info.roles.into_iter().map(RoleResponse::from).collect(),
        permissions: info.permissions.into_iter().map(PermissionResponse::from).collect(),
    }
}

pub fn to_menu_responses(menus: Vec<MenuInfo>) -> Vec<MenuResponse> {
    menus.into_iter().map(MenuResponse::from).collect()
}
