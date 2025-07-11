# Tradewinds API 文档

## 基础信息

- **Base URL**: `http://localhost:8080`
- **认证方式**: JWT Bearer Token
- **内容类型**: `application/json`

## 认证接口

### 用户注册
```http
POST /auth/register
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "password123",
  "real_name": "测试用户"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "注册成功",
  "data": {
    "user": {
      "id": 1,
      "username": "testuser",
      "email": "test@example.com",
      "real_name": "测试用户",
      "status": 1
    }
  }
}
```

### 用户登录
```http
POST /auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "admin123"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "username": "admin",
      "email": "admin@tradewinds.com"
    }
  }
}
```

### 用户登出
```http
POST /auth/logout
Authorization: Bearer {token}
```

### 获取超级管理员仪表盘
```http
GET /auth/super-admin/dashboard
Authorization: Bearer {token}
```

**描述**：获取超级管理员仪表盘数据，包括系统统计、用户统计、最近活动和系统健康状态。只有具有超级管理员权限的用户才能访问�?
**响应**:
```json
{
  "success": true,
  "data": {
    "system_stats": {
      "total_users": 1250,
      "active_users": 89,
      "total_roles": 8,
      "total_permissions": 45,
      "database_size": "2.3 GB",
      "uptime": "15 �?8 小时 32 分钟"
    },
    "user_stats": {
      "new_users_today": 12,
      "new_users_this_week": 87,
      "new_users_this_month": 324,
      "active_sessions": 156
    },
    "recent_activities": [
      {
        "id": "act_001",
        "user_id": "550e8400-e29b-41d4-a716-446655440001",
        "username": "admin",
        "action": "登录系统",
        "resource": "系统",
        "timestamp": 1703123456,
        "ip_address": "192.168.1.100",
        "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
      }
    ],
    "system_health": {
      "cpu_usage": 23.5,
      "memory_usage": 68.2,
      "disk_usage": 45.8,
      "database_status": "正常",
      "redis_status": "正常",
      "rabbitmq_status": "正常"
    }
  }
}
```

**字段说明**�?- `system_stats`: 系统统计信息
  - `total_users`: 总用户数
  - `active_users`: 活跃用户�?  - `total_roles`: 总角色数
  - `total_permissions`: 总权限数
  - `database_size`: 数据库大�?  - `uptime`: 系统运行时间
- `user_stats`: 用户统计信息
  - `new_users_today`: 今日新用�?  - `new_users_this_week`: 本周新用�?  - `new_users_this_month`: 本月新用�?  - `active_sessions`: 活跃会话�?- `recent_activities`: 最近活动记�?  - `id`: 活动ID
  - `user_id`: 用户ID
  - `username`: 用户�?  - `action`: 操作类型
  - `resource`: 操作资源
  - `timestamp`: 时间�?  - `ip_address`: IP地址
  - `user_agent`: 用户代理
- `system_health`: 系统健康状�?  - `cpu_usage`: CPU使用�?%)
  - `memory_usage`: 内存使用�?%)
  - `disk_usage`: 磁盘使用�?%)
  - `database_status`: 数据库状�?  - `redis_status`: Redis状�?  - `rabbitmq_status`: RabbitMQ状�?
### 获取用户菜单权限
```http
GET /auth/menus
Authorization: Bearer {token}
```

**描述**：获取当前用户的菜单权限，返回层级结构的菜单树，专门用于前端侧边栏渲染�?
**响应**:
```json
{
  "success": true,
  "data": {
    "menus": [
      {
        "id": "menu-1",
        "name": "系统管理",
        "code": "system",
        "path": "/system",
        "component": "Layout",
        "icon": "setting",
        "sort": 1,
        "parent_id": null,
        "children": [
          {
            "id": "menu-2",
            "name": "用户管理",
            "code": "system:user",
            "path": "/system/users",
            "component": "UserManagement",
            "icon": "user",
            "sort": 1,
            "parent_id": "menu-1",
            "children": []
          },
          {
            "id": "menu-3",
            "name": "角色管理",
            "code": "system:role",
            "path": "/system/roles",
            "component": "RoleManagement",
            "icon": "role",
            "sort": 2,
            "parent_id": "menu-1",
            "children": []
          }
        ]
      },
      {
        "id": "menu-4",
        "name": "业务管理",
        "code": "business",
        "path": "/business",
        "component": "Layout",
        "icon": "business",
        "sort": 2,
        "parent_id": null,
        "children": [
          {
            "id": "menu-5",
            "name": "订单管理",
            "code": "business:order",
            "path": "/business/orders",
            "component": "OrderManagement",
            "icon": "order",
            "sort": 1,
            "parent_id": "menu-4",
            "children": []
          }
        ]
      }
    ]
  }
}
```

**字段说明**�?- `id`: 菜单唯一标识
- `name`: 菜单显示名称
- `code`: 菜单权限代码
- `path`: 前端路由路径
- `component`: 前端组件名称
- `icon`: 菜单图标
- `sort`: 排序字段（数字越小越靠前�?- `parent_id`: 父菜单ID，null表示顶级菜单
- `children`: 子菜单数�?
## 用户管理接口

### 获取用户列表
```http
GET /users?page=1&page_size=10
Authorization: Bearer {token}
```

**响应**:
```json
{
  "code": 0,
  "message": "获取成功",
  "data": {
    "items": [
      {
        "id": 1,
        "username": "admin",
        "email": "admin@tradewinds.com",
        "real_name": "系统管理�?,
        "roles": ["超级管理�?]
      }
    ],
    "total": 1,
    "page": 1,
    "page_size": 10
  }
}
```

### 获取用户详情
```http
GET /users/{id}
Authorization: Bearer {token}
```

### 创建用户
```http
POST /users
Authorization: Bearer {token}
Content-Type: application/json

{
  "username": "newuser",
  "email": "new@example.com",
  "password": "password123",
  "real_name": "新用�?
}
```

### 更新用户
```http
PUT /users/{id}
Authorization: Bearer {token}
Content-Type: application/json

{
  "real_name": "更新的用户名",
  "phone": "13800138000"
}
```

### 删除用户
```http
DELETE /users/{id}
Authorization: Bearer {token}
```

## 角色管理接口

### 获取角色列表
```http
GET /roles?page=1&page_size=10
Authorization: Bearer {token}
```

### 获取角色详情
```http
GET /roles/{id}
Authorization: Bearer {token}
```

### 创建角色
```http
POST /roles
Authorization: Bearer {token}
Content-Type: application/json

{
  "name": "新角�?,
  "description": "角色描述"
}
```

### 分配权限给角�?```http
POST /roles/{id}/permissions
Authorization: Bearer {token}
Content-Type: application/json

{
  "permission_id": 1
}
```

## 权限管理接口

### 获取权限列表
```http
GET /permissions?page=1&page_size=10
Authorization: Bearer {token}
```

### 获取权限详情
```http
GET /permissions/{id}
Authorization: Bearer {token}
```

### 创建权限
```http
POST /permissions
Authorization: Bearer {token}
Content-Type: application/json

{
  "name": "新权�?,
  "code": "new:permission",
  "type": "menu",
  "path": "/new-permission"
}
```

## 错误码说�?
| 错误�?| 说明 |
|--------|------|
| 0 | 成功 |
| 1001 | 参数错误 |
| 1002 | 用户不存�?|
| 1003 | 密码错误 |
| 1004 | Token 无效 |
| 1005 | 权限不足 |
| 2001 | 用户名已存在 |
| 2002 | 邮箱已存�?|
| 5000 | 服务器内部错�?|

## 状态码

- `200`: 请求成功
- `400`: 请求参数错误
- `401`: 未认�?- `403`: 权限不足
- `404`: 资源不存�?- `500`: 服务器内部错�?