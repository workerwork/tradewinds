#[rustfmt::skip]
use crate::{
    QueryHandler,
    interfaces::auth_service::IAuthService,
    queries::auth::{GetCurrentUserQuery, get_user_menus_query::GetUserMenusQuery},
};
use crate::queries::auth::menu_info::MenuInfo;
use std::collections::HashMap;
use std::sync::Arc;
use tradewinds_domain::entities::permission::Permission;
use tradewinds_domain::value_objects::permission::PermissionType;
use tradewinds_error::AppResult;

fn build_children(parent_id: Option<String>, menu_map: &HashMap<String, MenuInfo>) -> Vec<MenuInfo> {
    let mut children = Vec::new();
    for menu in menu_map.values() {
        if menu.parent_id == parent_id {
            let mut child_menu = menu.clone();
            child_menu.children = build_children(Some(menu.id.clone()), menu_map);
            children.push(child_menu);
        }
    }
    children.sort_by_key(|m| m.sort);
    children
}

pub fn build_menu_tree(permissions: Vec<crate::queries::auth::user_info::PermissionInfo>) -> Vec<MenuInfo> {
    let mut menu_map: HashMap<String, MenuInfo> = HashMap::new();
    for permission in permissions {
        let menu = MenuInfo {
            id: permission.id.clone(),
            name: permission.name.clone(),
            code: permission.code.clone().unwrap_or_default(),
            path: permission.path.clone(),
            component: permission.component.clone(),
            icon: permission.icon.clone(),
            sort: permission.sort,
            parent_id: permission.parent_id.clone(),
            children: Vec::new(),
        };
        menu_map.insert(menu.id.clone(), menu);
    }
    build_children(None, &menu_map)
}

/// 获取用户菜单权限查询处理器
///
/// 参数：
/// - auth_service: 认证服务
///
/// 返回：
/// - 获取用户菜单权限查询处理器
pub struct GetUserMenusHandler {
    auth_service: Arc<dyn IAuthService>,
}

impl GetUserMenusHandler {
    pub fn new(auth_service: Arc<dyn IAuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl QueryHandler<GetUserMenusQuery, Vec<MenuInfo>> for GetUserMenusHandler {
    async fn handle(&self, query: GetUserMenusQuery) -> AppResult<Vec<MenuInfo>> {
        let get_user_query = GetCurrentUserQuery { token: query.token };
        let user_info = self.auth_service.get_current_user(get_user_query).await?;

        let mut menu_permissions: Vec<crate::queries::auth::user_info::PermissionInfo> = user_info
            .permissions
            .into_iter()
            .filter(|p| p.type_.eq_ignore_ascii_case("menu") || p.type_ == "0")
            .collect();

        menu_permissions.sort_by_key(|p| p.sort);

        // 如果 build_menu_tree 需要 PermissionInfo，则直接传；如果需要 Permission，需要额外转换。
        // 这里假设 build_menu_tree 也改为接收 Vec<PermissionInfo>
        Ok(build_menu_tree(menu_permissions))
    }
}
