#!/bin/bash

# 开发环境启动脚本

set -e

echo "🚀 启动 Tradewinds 开发环境..."

# 创建日志目录
mkdir -p logs

# 检查 .env 文件
if [ ! -f .env ]; then
    echo "📋 复制环境配置文件..."
    cp .env_example .env
    echo "⚠️  请编辑 .env 文件配置数据库等连接信息"
fi

# 检查数据库连接
echo "🔍 检查数据库连接..."
if ! cargo run --bin migrate --dry-run 2>/dev/null; then
    echo "❌ 数据库连接失败，请检查配置"
    exit 1
fi

# 运行数据库迁移
echo "📊 运行数据库迁移..."
cargo run --bin migrate

# 启动应用
echo "🎯 启动应用服务..."
cargo run

echo "✅ 开发环境启动完成！"
echo "🌐 访问地址: http://127.0.0.1:8080" 