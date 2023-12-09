# 使用官方 Rust 镜像作为基础镜像
FROM rust:1.71.0 as builder

# 设置工作目录
WORKDIR /app

# 复制整个项目到容器中
COPY . .
RUN rm Cargo.lock

# 构建应用程序
RUN cargo build --release

# 使用另一个基础镜像作为最终镜像
FROM debian:bullseye-slim

# 设置工作目录
WORKDIR /app

# 复制构建好的可执行文件到最终镜像中
COPY --from=builder /app/target/release/rust-web .
COPY configs /app/configs


ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8099

# 设置容器启动命令
CMD ["./rust-web"]

