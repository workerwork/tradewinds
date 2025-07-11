# 数据库操作命令手册

## 🔧 常用数据库操作命令

### 1. 数据库初始化

```bash
# 执行完整的数据库初始化脚本
mysqlsh --uri mysql://root:123456@localhost:3306 --sql -f scripts/database_init.sql
```

### 2. 用户数据查询

```bash
# 查看所有用户
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT id, username, email, real_name FROM users ORDER BY username;"

# 查看用户和角色对应关系
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT u.username, r.name FROM users u JOIN user_roles ur ON u.id = ur.user_id JOIN roles r ON ur.role_id = r.id;"

# 查看特定用户信息
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT * FROM users WHERE username = 'admin';"
```

### 3. 角色和权限查询

```bash
# 查看所有角色
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT id, name, description FROM roles;"

# 查看所有权限
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT id, name, code, type, parent_id, path FROM permissions ORDER BY sort;"

# 查看角色权限分配
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT r.name as role_name, p.name as permission_name, p.code FROM roles r JOIN role_permissions rp ON r.id = rp.role_id JOIN permissions p ON rp.permission_id = p.id ORDER BY r.name, p.sort;"

# 查看菜单层级结构（三级菜单）
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SELECT 
    CASE 
        WHEN p1.parent_id IS NULL THEN CONCAT('├─ ', p1.name)
        WHEN p2.parent_id IS NULL THEN CONCAT('│  ├─ ', p1.name)
        ELSE CONCAT('│  │  ├─ ', p1.name)
    END as menu_tree,
    p1.code,
    p1.path
FROM permissions p1
LEFT JOIN permissions p2 ON p1.parent_id = p2.id
LEFT JOIN permissions p3 ON p2.parent_id = p3.id
WHERE p1.type = 0
ORDER BY 
    COALESCE(p3.sort, p2.sort, p1.sort),
    COALESCE(p2.sort, p1.sort),
    p1.sort;"
```

### 4. 用户权限查询

```bash
# 查看特定用户的所有权限
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SELECT DISTINCT p.name, p.code, p.type, p.path 
FROM users u 
JOIN user_roles ur ON u.id = ur.user_id 
JOIN roles r ON ur.role_id = r.id 
JOIN role_permissions rp ON r.id = rp.role_id 
JOIN permissions p ON rp.permission_id = p.id 
WHERE u.username = 'admin' 
ORDER BY p.sort;"

# 查看特定用户的菜单权限
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SELECT DISTINCT p.name, p.code, p.path, p.component, p.icon, p.sort, p.parent_id
FROM users u 
JOIN user_roles ur ON u.id = ur.user_id 
JOIN roles r ON ur.role_id = r.id 
JOIN role_permissions rp ON r.id = rp.role_id 
JOIN permissions p ON rp.permission_id = p.id 
WHERE u.username = 'manager' AND p.type = 0 
ORDER BY p.sort;"
```

### 5. 数据库连接和基本操作

```bash
# 连接到MySQL服务器
mysqlsh --uri mysql://root:123456@localhost:3306

# 连接到特定数据库
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds

# 执行单条SQL命令
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SHOW TABLES;"

# 执行SQL文件
mysqlsh --uri mysql://root:123456@localhost:3306 --sql -f path/to/script.sql
```

### 6. 数据清理和重置

```bash
# 清空所有表数据（保留表结构）
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SET FOREIGN_KEY_CHECKS = 0;
TRUNCATE TABLE token_blacklist;
TRUNCATE TABLE role_permissions;
TRUNCATE TABLE user_roles;
TRUNCATE TABLE permissions;
TRUNCATE TABLE roles;
TRUNCATE TABLE users;
SET FOREIGN_KEY_CHECKS = 1;"

# 删除整个数据库
mysqlsh --uri mysql://root:123456@localhost:3306 --sql -e "DROP DATABASE IF EXISTS tradewinds;"
```

### 7. 测试数据验证

```bash
# 验证三个默认用户是否存在
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SELECT 
    u.username,
    u.email,
    u.real_name,
    r.name as role_name
FROM users u 
LEFT JOIN user_roles ur ON u.id = ur.user_id 
LEFT JOIN roles r ON ur.role_id = r.id 
ORDER BY u.username;"

# 检查权限分配是否正确
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "
SELECT 
    r.name as role_name,
    COUNT(rp.permission_id) as permission_count
FROM roles r 
LEFT JOIN role_permissions rp ON r.id = rp.role_id 
GROUP BY r.id, r.name 
ORDER BY r.name;"
```

## 📝 默认账号信息

| 用户名 | 密码 | 角色 | 权限说明 |
|--------|------|------|----------|
| `admin` | `admin123` | 超级管理员 | 拥有所有权限，包括超级管理员控制台 |
| `manager` | `admin123` | 普通管理员 | 拥有系统管理权限，不包括超级管理员控制台 |
| `user` | `admin123` | 普通用户 | 只有基础查看权限 |

## ⚠️ 注意事项

1. **密码安全**: 生产环境中请修改默认密码
2. **备份**: 执行清理操作前请先备份数据
3. **权限**: 确保MySQL用户有足够的权限执行这些操作
4. **网络**: 命令中的localhost:3306需要根据实际环境调整

## 🔄 常见操作场景

### 重新初始化数据库
```bash
# 1. 删除旧数据库
mysqlsh --uri mysql://root:123456@localhost:3306 --sql -e "DROP DATABASE IF EXISTS tradewinds;"

# 2. 重新初始化
mysqlsh --uri mysql://root:123456@localhost:3306 --sql -f scripts/database_init.sql
```

### 添加新用户
```bash
# 查看现有用户ID模式，然后手动插入新用户
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT id FROM users LIMIT 1;"
```

### 调试权限问题
```bash
# 检查用户是否存在
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT * FROM users WHERE username = 'your_username';"

# 检查用户角色
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT ur.*, r.name FROM user_roles ur JOIN roles r ON ur.role_id = r.id WHERE ur.user_id = 'your_user_id';"

# 检查角色权限
mysqlsh --uri mysql://root:123456@localhost:3306/tradewinds --sql -e "SELECT rp.*, p.name FROM role_permissions rp JOIN permissions p ON rp.permission_id = p.id WHERE rp.role_id = 'your_role_id';"
```

## 📋 菜单结构说明

### 新的三级菜单结构
```
├─ 超级管理员
│  ├─ 管理员仪表盘
│  ├─ 系统监控
│  ├─ 数据备份
│  ├─ 系统配置
│  └─ 操作日志
├─ 系统管理
│  └─ 用户管理
│     ├─ 用户列表
│     ├─ 角色管理
│     └─ 权限管理
```

### 结构变更说明
- **原来的"用户管理"** 改名为 **"用户列表"**
- **新增"用户管理"** 作为二级菜单
- **用户列表、角色管理、权限管理** 都移至新的"用户管理"下作为三级菜单
- 路径结构相应调整：
  - 用户列表：`/system/user-management/users`
  - 角色管理：`/system/user-management/roles`  
  - 权限管理：`/system/user-management/permissions`