#!/bin/bash

# 生产环境部署脚本

set -e

APP_NAME="tradewinds"
VERSION=${1:-latest}
DOCKER_IMAGE="tradewinds:${VERSION}"

echo "🚀 开始部署 ${APP_NAME} ${VERSION}..."

# 构建 Docker 镜像
echo "🔨 构建 Docker 镜像..."
docker build -t ${DOCKER_IMAGE} .

# 停止现有容器
echo "⏹️  停止现有容器..."
docker-compose down || true

# 启动新容器
echo "▶️  启动新容器..."
docker-compose up -d

# 等待服务启动
echo "⏳ 等待服务启动..."
sleep 10

# 健康检查
echo "🔍 进行健康检查..."
if curl -f http://localhost:8080/health > /dev/null 2>&1; then
    echo "✅ 部署成功！服务正常运行"
else
    echo "❌ 部署失败！服务无法访问"
    docker-compose logs app
    exit 1
fi

# 清理旧镜像
echo "🧹 清理旧镜像..."
docker image prune -f

echo "🎉 部署完成！"
echo "🌐 访问地址: http://localhost:8080" 