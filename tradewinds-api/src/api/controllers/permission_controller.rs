use std::sync::Arc;

use tradewinds_application::commands::permission::handlers::{
    CreatePermissionHandler, DeletePermissionHandler, UpdatePermissionHandler,
};
use tradewinds_application::commands::permission::*;
use tradewinds_application::interfaces::IPermissionService;
use tradewinds_application::queries::permission::handlers::{
    GetPermissionByCodeHandler, GetPermissionByIdHandler, GetPermissionByNameHandler, ListPermissionsByParentIdHandler,
    ListPermissionsByTypeHandler, ListPermissionsHandler,
};
use tradewinds_application::queries::permission::list_all_permissions_query::ListAllPermissionsQuery;
use tradewinds_application::queries::permission::*;
use tradewinds_application::{CommandHandler, QueryHandler};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_error::AppResult;

#[rustfmt::skip]
use crate::api::{
    dtos::permission_dto::*,
    mappers::permission_mapper,
};

/// 权限控制器
pub struct PermissionController {
    create_permission: Arc<dyn CommandHandler<CreatePermissionCommand, Permission>>,
    update_permission: Arc<dyn CommandHandler<UpdatePermissionCommand, ()>>,
    delete_permission: Arc<dyn CommandHandler<DeletePermissionCommand, ()>>,
    get_permission_by_id: Arc<dyn QueryHandler<GetPermissionByIdQuery, Permission>>,
    get_permission_by_name: Arc<dyn QueryHandler<GetPermissionByNameQuery, Permission>>,
    get_permission_by_code: Arc<dyn QueryHandler<GetPermissionByCodeQuery, Permission>>,
    list_permissions: Arc<dyn QueryHandler<ListPermissionsQuery, PaginatedResult<Permission>>>,
    list_permissions_by_type: Arc<dyn QueryHandler<ListPermissionsByTypeQuery, PaginatedResult<Permission>>>,
    list_permissions_by_parent_id: Arc<dyn QueryHandler<ListPermissionsByParentIdQuery, PaginatedResult<Permission>>>,
    list_all_permissions: Arc<dyn QueryHandler<ListAllPermissionsQuery, Vec<Permission>>>,
}

impl PermissionController {
    pub fn new(
        create_permission: Arc<dyn CommandHandler<CreatePermissionCommand, Permission>>,
        update_permission: Arc<dyn CommandHandler<UpdatePermissionCommand, ()>>,
        delete_permission: Arc<dyn CommandHandler<DeletePermissionCommand, ()>>,
        get_permission_by_id: Arc<dyn QueryHandler<GetPermissionByIdQuery, Permission>>,
        get_permission_by_name: Arc<dyn QueryHandler<GetPermissionByNameQuery, Permission>>,
        get_permission_by_code: Arc<dyn QueryHandler<GetPermissionByCodeQuery, Permission>>,
        list_permissions: Arc<dyn QueryHandler<ListPermissionsQuery, PaginatedResult<Permission>>>,
        list_permissions_by_type: Arc<dyn QueryHandler<ListPermissionsByTypeQuery, PaginatedResult<Permission>>>,
        list_permissions_by_parent_id: Arc<
            dyn QueryHandler<ListPermissionsByParentIdQuery, PaginatedResult<Permission>>,
        >,
        list_all_permissions: Arc<dyn QueryHandler<ListAllPermissionsQuery, Vec<Permission>>>,
    ) -> Self {
        Self {
            create_permission,
            update_permission,
            delete_permission,
            get_permission_by_id,
            get_permission_by_name,
            get_permission_by_code,
            list_permissions,
            list_permissions_by_type,
            list_permissions_by_parent_id,
            list_all_permissions,
        }
    }

    pub fn assemble(permission_service: Arc<dyn IPermissionService>) -> Self {
        Self::new(
            Arc::new(CreatePermissionHandler::new(permission_service.clone())),
            Arc::new(UpdatePermissionHandler::new(permission_service.clone())),
            Arc::new(DeletePermissionHandler::new(permission_service.clone())),
            Arc::new(GetPermissionByIdHandler::new(permission_service.clone())),
            Arc::new(GetPermissionByNameHandler::new(permission_service.clone())),
            Arc::new(GetPermissionByCodeHandler::new(permission_service.clone())),
            Arc::new(ListPermissionsHandler::new(permission_service.clone())),
            Arc::new(ListPermissionsByTypeHandler::new(permission_service.clone())),
            Arc::new(ListPermissionsByParentIdHandler::new(permission_service.clone())),
            Arc::new(ListAllPermissionsHandler::new(permission_service.clone())),
        )
    }

    pub async fn create_permission(
        &self,
        actor_id: String,
        req: CreatePermissionRequest,
    ) -> AppResult<CreatePermissionResponse> {
        let command = permission_mapper::to_create_permission_command(actor_id, req)?;
        let permission = self.create_permission.handle(command).await?;
        Ok(CreatePermissionResponse { permission: permission.into() })
    }

    pub async fn update_permission(
        &self,
        actor_id: String,
        req: UpdatePermissionRequest,
    ) -> AppResult<UpdatePermissionResponse> {
        let command = permission_mapper::to_update_permission_command(actor_id, req)?;
        let _ = self.update_permission.handle(command).await?;
        Ok(UpdatePermissionResponse)
    }

    pub async fn delete_permission(
        &self,
        actor_id: String,
        req: DeletePermissionRequest,
    ) -> AppResult<DeletePermissionResponse> {
        let command = permission_mapper::to_delete_permission_command(actor_id, req)?;
        let _ = self.delete_permission.handle(command).await?;
        Ok(DeletePermissionResponse)
    }

    pub async fn get_permission_by_id(&self, req: GetPermissionByIdRequest) -> AppResult<GetPermissionByIdResponse> {
        let query = permission_mapper::to_get_permission_by_id_query(req)?;
        let permission = self.get_permission_by_id.handle(query).await?;
        Ok(GetPermissionByIdResponse { permission: permission.into() })
    }

    pub async fn get_permission_by_name(
        &self,
        req: GetPermissionByNameRequest,
    ) -> AppResult<GetPermissionByNameResponse> {
        let query = permission_mapper::to_get_permission_by_name_query(req)?;
        let permission = self.get_permission_by_name.handle(query).await?;
        Ok(GetPermissionByNameResponse { permission: permission.into() })
    }

    pub async fn get_permission_by_code(
        &self,
        req: GetPermissionByCodeRequest,
    ) -> AppResult<GetPermissionByCodeResponse> {
        let query = permission_mapper::to_get_permission_by_code_query(req)?;
        let permission = self.get_permission_by_code.handle(query).await?;
        Ok(GetPermissionByCodeResponse { permission: permission.into() })
    }

    pub async fn list_permissions(&self, req: ListPermissionsRequest) -> AppResult<ListPermissionsResponse> {
        let query = permission_mapper::to_list_permissions_query(req)?;
        let result = self.list_permissions.handle(query).await?;
        Ok(ListPermissionsResponse {
            permissions: result.items.into_iter().map(Into::into).collect(),
            total: result.total,
        })
    }

    pub async fn list_permissions_by_type(
        &self,
        req: ListPermissionsByTypeRequest,
    ) -> AppResult<ListPermissionsByTypeResponse> {
        let query = permission_mapper::to_list_permissions_by_type_query(req)?;
        let result = self.list_permissions_by_type.handle(query).await?;
        Ok(ListPermissionsByTypeResponse {
            permissions: result.items.into_iter().map(Into::into).collect(),
            total: result.total,
        })
    }

    pub async fn list_permissions_by_parent_id(
        &self,
        req: ListPermissionsByParentIdRequest,
    ) -> AppResult<ListPermissionsByParentIdResponse> {
        let query = permission_mapper::to_list_permissions_by_parent_id_query(req)?;
        let result = self.list_permissions_by_parent_id.handle(query).await?;
        Ok(ListPermissionsByParentIdResponse {
            permissions: result.items.into_iter().map(Into::into).collect(),
            total: result.total,
        })
    }

    pub async fn list_all_permissions(&self) -> AppResult<Vec<PermissionTreeResponse>> {
        let flat = self.list_all_permissions.handle(ListAllPermissionsQuery).await?;
        Ok(permission_mapper::to_tree_responses(flat))
    }
}
