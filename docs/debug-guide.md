# 调试指南

## 🔧 调试环境配置

### 1. 环境变量设置
在 `.env` 文件中设置以下调试相关配置：

```bash
# 日志级别（调试时使用 debug）
LOG_LEVEL=debug
RUST_LOG=debug,sqlx=info,sea_orm=debug,tradewinds=debug
RUST_BACKTRACE=1

# 启用详细的数据库日志
DATABASE_LOG_LEVEL=debug
```

### 2. 启动调试模式
```bash
# 开发环境启动（带调试信息）
RUST_LOG=debug cargo run

# 或者使用脚本
./scripts/dev.sh
```

## 📝 错误日志记录

### 自动错误记录
项目已经配置了自动错误记录，所有 `AppError` 都会自动记录：

```rust
// 错误会自动记录到日志
return Err(AppError::Validation("用户名不能为空".to_string()));

// 日志输出：
// WARN Validation error: 用户名不能为空
// ERROR Error response: status=400, error=Validation("用户名不能为空")
```

### 使用调试宏

#### 1. 函数执行跟踪
```rust
use tradewinds_common::{debug_fn, debug_fn_exit, time_it};

pub async fn create_user(req: CreateUserRequest) -> AppResult<User> {
    debug_fn!("create_user", &format!("username: {}", req.username));
    
    let result = time_it!("用户创建", {
        // 业务逻辑
        user_service.create(req).await
    });
    
    debug_fn_exit!("create_user", "success");
    result
}
```

#### 2. 错误详细记录
```rust
use tradewinds_common::debug_error;

match user_repository.find_by_id(user_id).await {
    Ok(user) => Ok(user),
    Err(e) => {
        debug_error!(e, "查找用户失败", "UserService::find_by_id");
        Err(AppError::Database(format!("用户查找失败: {}", e)))
    }
}
```

#### 3. 性能监控
```rust
use tradewinds_common::time_it;

let users = time_it!("获取用户列表", {
    user_repository.list_with_pagination(page, page_size).await?
});

// 输出：⚡ 性能: 获取用户列表 耗时 45ms
// 或者：🐌 性能警告: 获取用户列表 耗时 1200ms
```

## 🗃️ 数据库调试

### 1. 启用 SQL 日志
在 `.env` 中设置：
```bash
RUST_LOG=sqlx=debug
```

### 2. 查看 SQL 执行
```rust
use tradewinds_common::debug::DebugHelper;

DebugHelper::db_operation("SELECT", "users", Some("查询活跃用户"));

let users = sqlx::query_as!(User, "SELECT * FROM users WHERE status = ?", 1)
    .fetch_all(&pool)
    .await?;

// 输出：🗄️ 数据库操作: SELECT on users - 查询活跃用户
```

## 📊 日志级别说明

| 级别 | 用途 | 示例 |
|------|------|------|
| `ERROR` | 系统错误、数据库错误 | 🔴 严重错误需要立即处理 |
| `WARN` | 业务警告、认证失败 | ⚠️ 需要关注但不影响系统运行 |
| `INFO` | 业务操作、重要事件 | ℹ️ 正常的业务流程记录 |
| `DEBUG` | 函数调用、详细信息 | 🔍 开发调试信息 |

## 🛠️ 调试技巧

### 1. 快速定位错误
```bash
# 只查看错误日志
tail -f logs/app.log | grep ERROR

# 查看特定模块的日志
RUST_LOG=tradewinds_application=debug cargo run
```

### 2. 结构化日志查询
```bash
# 查看数据库相关错误
tail -f logs/app.log | grep "Database error"

# 查看认证相关问题
tail -f logs/app.log | grep "Authentication"
```

### 3. 性能分析
```bash
# 查看性能警告
tail -f logs/app.log | grep "性能警告"

# 查看慢查询
tail -f logs/app.log | grep "耗时.*ms" | grep -E "[0-9]{4,}ms"
```

## 🚨 常见调试场景

### 1. 用户认证失败
```rust
// 在认证中间件中查看详细信息
warn!("Authentication failed: token={}, reason={}", token, reason);
```

### 2. 数据库连接问题
```rust
// 数据库操作前后记录
debug!("尝试连接数据库: {}", database_url);
match connection_result {
    Ok(_) => info!("数据库连接成功"),
    Err(e) => error!("数据库连接失败: {:?}", e),
}
```

### 3. 业务逻辑错误
```rust
// 业务规则验证
if user.status != UserStatus::Active {
    warn!("用户状态异常: user_id={}, status={:?}", user.id, user.status);
    return Err(AppError::Business("用户账户已被禁用".to_string()));
}
```

## 📈 监控建议

1. **生产环境**: 使用 `LOG_LEVEL=info`，减少日志量
2. **测试环境**: 使用 `LOG_LEVEL=debug`，便于问题排查
3. **开发环境**: 使用 `LOG_LEVEL=debug` + `RUST_BACKTRACE=1`

## 🔗 相关工具

- **日志查看**: `tail`, `grep`, `less`
- **性能分析**: 内置性能监控宏
- **错误追踪**: 自动堆栈跟踪（`RUST_BACKTRACE=1`） 