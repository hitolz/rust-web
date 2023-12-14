# Rust web 开发-8.docker

本系列文章从以下几个方面学习如何使用 Rust 进行 web 开发。

1. web 框架
2. 数据库/orm
3. config
4. log
5. 线程池
6. kafka
7. redis
8. 打包成 docker 镜像
   ……

---
本篇介绍一下 Rust 项目如何打包成 docker 镜像。

## 基础 Dockerfile

在项目的根目录创建 Dockerfile

```dockerfile
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

EXPOSE 8099

# 设置容器启动命令
CMD ["./rust-web"]
```
### docker build
```
docker build -t rust-web .
```

### docker run
```
docker run -d -p 8099:8099 -v path_to_configs/:/app/configs/ --name rust-web rust-web
```

注意启动时指定配置文件路径。

## 修改配置文件
config.toml 中修改各个中间件的地址信息，不再是 localhost。
需要注意一点，app 的 host 指定为 0.0.0.0，接收外部所有 ip 的访问请求。

```
[app]
host="0.0.0.0"
port=8099

[database]
host="192.168.0.116"
port=3306
user = "root"
password = "12345678"
name = "rust_web"


[log]
level = "info"
path = "app.log"


[kafka_config]
brokers = "192.168.0.116:9092"
group_id = "test_group"

[redis_config]
cluster = "redis://192.168.0.116:6379/"
```

## 加快 docker build 速度

使用 `cargo-chef` 加速 build。
`cargo-chef` 是一个 Rust 工具，用于加速构建过程。它可以预先计算构建过程中的依赖项，以减少重复工作。通过生成一个描述项目依赖项的 `recipe.json` 文件，`cargo-chef` 可以在后续的构建中利用这个文件来加速构建过程。


```
# 使用官方 Rust 镜像作为基础镜像，并添加 cargo-chef
FROM rust:1.71.0 as chef
ADD .cargo $CARGO_HOME/
RUN cargo install cargo-chef

FROM chef as planner

WORKDIR /app
# 复制整个项目到容器中
COPY . .
RUN rm Cargo.lock

# 生成 recipe.json
RUN cargo chef prepare --recipe-path recipe.json

# 构建阶段
FROM chef as builder
WORKDIR /app

# 复制项目和 recipe.json
COPY --from=planner /app/recipe.json recipe.json
COPY . .
RUN rm Cargo.lock

# 使用 cargo-chef 加速构建过程
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
```
相比上一个版本，这个 Dockerfile 做了以下修改：

1. 添加了一个中间镜像 `chef`：
    - 使用官方 Rust 镜像作为基础镜像，并将其命名为 `chef`。
    - 将 `.cargo` 目录添加到 `$CARGO_HOME`（通常为 `/usr/local/cargo`）中，以便在后续的构建阶段中使用。
    - 安装了 `cargo-chef` 工具。

2. `planner` 阶段：
    - 将 `chef` 阶段作为基础镜像，并将其命名为 `planner`。
    - 在工作目录 `/app` 下复制整个项目。
    - 运行 `cargo chef prepare` 命令生成一个 `recipe.json` 文件。

3. `builder` 阶段：
    - 将 `chef` 阶段作为基础镜像，并将其命名为 `builder`。
    - 在工作目录 `/app` 下复制整个项目和 `recipe.json` 文件。
    - 运行 `cargo chef cook` 命令根据 `recipe.json` 文件来加速构建过程。
    - 运行 `cargo build --release` 命令构建应用程序。

这些修改的好处是：

1. 引入了中间镜像 `chef`，将 `cargo-chef` 安装过程提前到构建过程的早期阶段。这样可以避免在每次构建时都重新安装 `cargo-chef`，从而提高构建效率。

2. 将 `cargo-chef` 的安装和配置步骤从 `planner` 和 `builder` 阶段中移除，使得这两个阶段更加简洁和清晰。

3. 在 `planner` 阶段中，将 `.cargo` 目录添加到 `$CARGO_HOME` 中，以便在后续的构建阶段中使用。这样可以确保在 `builder` 阶段中可以访问到预先安装的 `cargo-chef` 工具，而无需重新安装。

总体而言，这些修改使得构建过程更加高效和简洁。通过引入中间镜像和提前安装 `cargo-chef`，可以减少重复工作并加快构建速度。同时，将 `cargo-chef` 的安装和配置步骤从每个阶段中移除，使得 Dockerfile 更加易读和易于维护。

本文代码在 [github](https://github.com/hitolz/rust-web/tree/docker)。


## 小结
以上就是将 Rust web 项目打包为 docker 镜像的过程，通过 `cargo-chef`可以加速 docker build 过程。
打包成 docker 镜像以后，部署到各种云平台都会方便很多。




