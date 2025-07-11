# 构建阶段
FROM rust:1.80-slim as builder

# 安装必要的构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./
COPY tradewinds-*/Cargo.toml ./

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# 创建应用用户
RUN useradd -m -u 1001 app

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/tradewinds /app/
COPY --from=builder /app/target/release/migrate /app/
COPY --from=builder /app/target/release/hash_password /app/

# 复制配置文件
COPY .env_example /app/.env

# 创建日志目录
RUN mkdir -p /app/logs && chown -R app:app /app

# 切换到应用用户
USER app

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动应用
CMD ["./tradewinds"] 