use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    domain::{
        repositories::user_repository::UserRepository,
        services::permission_service::PermissionService,
    },
    shared::types::AppError,
};

use tradewinds_error::AppError;

pub struct RequirePermission(pub String);

pub async fn permission_middleware<U: UserRepository, P: PermissionService>(
    State(user_repository): State<Arc<U>>,
    State(permission_service): State<Arc<P>>,
    require_permission: RequirePermission,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let user_id = request
        .extensions()
        .get::<String>()
        .ok_or_else(|| AppError::Auth("Missing user ID in request".to_string()))?
        .clone();

    let user = user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with id: {}", user_id)))?;

    let has_permission = permission_service.has_permission(&user, &require_permission.0).await?;

    if !has_permission {
        return Err(AppError::Auth("Permission denied".to_string()));
    }

    Ok(next.run(request).await)
}

pub struct RequireRole(pub String);

pub async fn role_middleware<U: UserRepository, P: PermissionService>(
    State(user_repository): State<Arc<U>>,
    State(permission_service): State<Arc<P>>,
    require_role: RequireRole,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let user_id = request
        .extensions()
        .get::<String>()
        .ok_or_else(|| AppError::Auth("Missing user ID in request".to_string()))?
        .clone();

    let user = user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User not found with id: {}", user_id)))?;

    let has_role = permission_service.has_role(&user, &require_role.0).await?;

    if !has_role {
        return Err(AppError::Auth("Role required".to_string()));
    }

    Ok(next.run(request).await)
}
