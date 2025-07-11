use crate::commands::permission::{CreatePermissionCommand, DeletePermissionCommand, UpdatePermissionCommand};
use crate::interfaces::IPermissionService;
use crate::queries::{
    get_permission_by_id_query::GetPermissionByIdQuery, get_permission_by_name_query::GetPermissionByNameQuery,
    list_permissions_by_parent_id_query::ListPermissionsByParentIdQuery,
    list_permissions_by_type_query::ListPermissionsByTypeQuery, list_permissions_query::ListPermissionsQuery,
};
use tradewinds_common::PaginatedResult;
use tradewinds_domain::aggregates::permission_aggregate::PermissionAggregate;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_domain::repositories::{PermissionAggregateRepository, PermissionRepository};

use std::sync::Arc;
use tradewinds_error::{AppError, AppResult};

#[derive(Clone)]
pub struct PermissionService {
    permission_repo: Arc<dyn PermissionRepository>,
    permission_agg_repo: Arc<dyn PermissionAggregateRepository>,
}

impl PermissionService {
    pub fn new(
        permission_repo: Arc<dyn PermissionRepository>,
        permission_agg_repo: Arc<dyn PermissionAggregateRepository>,
    ) -> Self {
        Self { permission_repo, permission_agg_repo }
    }
}

#[async_trait::async_trait]
impl IPermissionService for PermissionService {
    async fn list_permissions(&self, query: ListPermissionsQuery) -> AppResult<PaginatedResult<Permission>> {
        let offset = (query.page - 1) * query.page_size;
        let (list, total) = self
            .permission_repo
            .search(
                query.name.as_ref(),
                query.code.as_ref(),
                query.permission_type.as_ref(),
                query.status,
                query.show_deleted,
                query.page_size,
                offset,
            )
            .await?;
        Ok(PaginatedResult { items: list, total })
    }

    async fn list_permissions_by_type(
        &self,
        query: ListPermissionsByTypeQuery,
    ) -> AppResult<PaginatedResult<Permission>> {
        let (list, total) =
            self.permission_repo.search(None, None, Some(&query.permission_type), None, None, 100, 0).await?;
        Ok(PaginatedResult { items: list, total })
    }

    async fn list_permissions_by_parent_id(
        &self,
        query: ListPermissionsByParentIdQuery,
    ) -> AppResult<PaginatedResult<Permission>> {
        let (list, _total) = self.permission_repo.search(None, None, None, None, None, 100, 0).await?;
        let filtered: Vec<_> = list.into_iter().filter(|p| p.parent_id.as_ref() == Some(&query.parent_id)).collect();
        let total = filtered.len() as u64;
        Ok(PaginatedResult { items: filtered, total })
    }

    async fn create_permission(&self, cmd: CreatePermissionCommand) -> AppResult<Permission> {
        if let Some(code) = &cmd.code {
            if self.permission_repo.find_by_code(code).await?.is_some() {
                return Err(AppError::Validation("Permission code already exists".into()));
            }
        }

        let permission_agg = PermissionAggregate::create(
            cmd.name,
            cmd.code,
            cmd.type_,
            cmd.parent_id,
            cmd.path,
            cmd.component,
            cmd.icon,
            cmd.sort,
        )?;

        self.permission_agg_repo.create(&permission_agg).await?;

        Ok(permission_agg.permission)
    }

    async fn update_permission(&self, cmd: UpdatePermissionCommand) -> AppResult<()> {
        let mut permission_agg = self
            .permission_agg_repo
            .find_by_id(&cmd.id)
            .await?
            .ok_or_else(|| AppError::NotFound("Permission not found".into()))?;

        // 处理 parent_id 逻辑
        let parent_id = match cmd.parent_id {
            None => None,                                   // 不修改
            Some(None) => Some(None),                       // 清空
            Some(Some(ref pid)) => Some(Some(pid.clone())), // 设置为指定父权限
        };

        permission_agg.update(
            cmd.name,
            cmd.code,
            cmd.type_,
            parent_id,
            cmd.path,
            cmd.component,
            cmd.icon,
            cmd.sort,
            cmd.status,
        )?;

        self.permission_agg_repo.save(&permission_agg).await?;

        Ok(())
    }

    async fn delete_permission(&self, cmd: DeletePermissionCommand) -> AppResult<()> {
        // 检查是否有子权限
        let (list, _total) = self.permission_repo.search(None, None, None, None, None, 100, 0).await?;
        let children_vec: Vec<_> =
            list.into_iter().filter(|p| p.parent_id.as_ref() == Some(&cmd.permission_id)).collect();
        if !children_vec.is_empty() {
            return Err(AppError::Validation("Cannot delete permission with children".into()));
        }

        self.permission_agg_repo.delete_by_id(&cmd.permission_id).await?;
        Ok(())
    }

    async fn get_permission_by_id(&self, query: GetPermissionByIdQuery) -> AppResult<Permission> {
        self.permission_repo
            .find_by_id(&query.permission_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Permission not found".into()))
    }

    async fn get_permission_by_name(&self, query: GetPermissionByNameQuery) -> AppResult<Permission> {
        self.permission_repo
            .find_by_name(&query.permission_name)
            .await?
            .ok_or_else(|| AppError::NotFound("Permission not found".into()))
    }

    async fn get_permission_by_code(
        &self,
        query: crate::queries::get_permission_by_code_query::GetPermissionByCodeQuery,
    ) -> AppResult<Permission> {
        self.permission_repo
            .find_by_code(&query.permission_code)
            .await?
            .ok_or_else(|| AppError::NotFound("Permission not found".into()))
    }

    async fn list_all_permissions(&self) -> AppResult<Vec<Permission>> {
        self.permission_repo.find_all().await
    }
}
