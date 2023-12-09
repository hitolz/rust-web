# 使用官方 Rust 镜像作为基础镜像，并添加 cargo-chef
FROM rust:1.71.0 as planner
WORKDIR /app
RUN cargo install cargo-chef
# 复制整个项目到容器中
COPY . .
RUN rm Cargo.lock

# 生成 recipe.json
RUN cargo chef prepare --recipe-path recipe.json

# 构建阶段
FROM rust:1.71.0 as builder
WORKDIR /app

# 复制项目和 recipe.json
COPY --from=planner /app/recipe.json recipe.json
COPY . .
RUN rm Cargo.lock

# 使用 cargo-chef 加速构建过程
RUN cargo install cargo-chef
RUN cargo chef cook --release --recipe-path recipe.json

# 构建应用程序
RUN cargo build --release


# 使用另一个基础镜像作为最终镜像
FROM debian:bullseye-slim
# 设置工作目录
WORKDIR /app

# 复制构建好的可执行文件到最终镜像中
COPY --from=builder /app/target/release/rust-web .
COPY configs /app/configs

EXPOSE 8099

# 设置容器启动命令
CMD ["./rust-web"]

