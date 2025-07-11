# Tradewinds

一个基于 DDD（领域驱动设计）架构的 Rust 后端服务项目。

## 🏗️ 项目架构

本项目采用洋葱架构（Onion Architecture）和 DDD 设计模式，分为以下模块：

```
tradewinds/
├── tradewinds-api/          # API 层 - HTTP/GraphQL/gRPC/WebSocket 接口
├── tradewinds-application/  # 应用层 - 应用服务、命令查询处理器
├── tradewinds-domain/       # 领域层 - 实体、聚合根、领域服务
├── tradewinds-infrastructure/ # 基础设施层 - 数据库、缓存、消息队列
├── tradewinds-common/       # 通用工具库
└── tradewinds-error/        # 错误处理模块
```

## 🚀 功能特性

- ✅ 用户管理（注册、登录、权限控制）
- ✅ 角色权限系统（RBAC）
- ✅ JWT 认证授权
- ✅ 多接口支持（HTTP REST API）
- ✅ 数据库迁移
- ✅ 缓存集成（Redis）
- ✅ 消息队列（RabbitMQ）
- ✅ 密码加密
- ✅ 请求日志记录

## 🛠️ 技术栈

- **语言**: Rust 2024 Edition
- **Web 框架**: Axum
- **数据库**: MySQL + SeaORM
- **缓存**: Redis
- **消息队列**: RabbitMQ
- **认证**: JWT + Bcrypt
- **日志**: Tracing
- **异步运行时**: Tokio

## 📦 快速开始

### 环境要求

- Rust 1.75+
- MySQL 8.0+
- Redis 6.0+
- RabbitMQ 3.8+

### 安装配置

1. **克隆项目**
```bash
git clone https://github.com/your-username/tradewinds.git
cd tradewinds
```

2. **环境配置**
```bash
cp .env_example .env
# 编辑 .env 文件，配置数据库等连接信息
```

3. **数据库迁移**
```bash
cargo run --bin migrate
```

4. **运行项目**
```bash
cargo run
```

### 使用工具

**生成密码哈希**
```bash
cargo run --bin hash_password
```

## 🔧 开发指南

### 目录结构说明

- `tradewinds-api/`: API 层，包含控制器、路由、DTO、中间件
- `tradewinds-application/`: 应用层，包含命令/查询处理器和应用服务
- `tradewinds-domain/`: 领域层，包含实体、聚合根、领域服务、规约
- `tradewinds-infrastructure/`: 基础设施层，包含数据库、缓存、消息队列实现

### 编码规范

项目使用 `rustfmt` 进行代码格式化：
```bash
cargo fmt
```

### 测试

```bash
cargo test
```

## 📚 API 文档

启动服务后，访问以下端点：

- 健康检查: `GET /health`
- 用户注册: `POST /api/auth/register`
- 用户登录: `POST /api/auth/login`
- 获取用户列表: `GET /api/users`

## 🤝 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 👥 维护者

- workerwork <workerwork@qq.com> 