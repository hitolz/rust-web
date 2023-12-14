# Rust web 开发-3.config


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
本文介绍第三部分，Rust 如何读取配置文件。
Rust 支持两种类型的配置文件，`.toml` 和 `.env`。

本文主要介绍 toml 类型的文件。

## 配置文件

项目中新建一个 toml 类型的文件，例如：config.toml，放在某一个文件夹下，本文示例代码是在与 src 同级的目录 configs 下。
```toml
[app]
host="127.0.0.1"
port=8099

[database]
host="localhost"
port=3306
user = "root"
password = "12345678"
name = "rust_web"
```


## 定义实体
```rust
/// 主机,端口
#[derive(Deserialize, Default, Debug, Clone)]
pub struct App {
    pub host: String,
    pub port: u16,
}

/// 数据库连接信息
#[derive(Deserialize, Default, Debug, Clone)]
pub struct Database {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: usize,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct ServerConfig {
    pub app: App,
    pub database: Database,
}

// 为 ServerConfig 实体提供一些方法，可以比较方便的获取配置文件具体信息。
impl ServerConfig {
    // 使用引用比较方便，可以避免一些所有权转移问题
    pub fn database_url(&self) -> String {
        let db = &self.database;
        // "mysql://root:12345678@localhost:3306/rust_web"
        format!(
            "mysql://{}:{}@{}:{}/{}",
            db.user, db.password, db.host, db.port, db.name
        )
    }
    pub fn get_app_host_port(&self) -> (&str, u16) {
        (&self.app.host, self.app.port)
    }
}
```

如果有新增的配置内容，要同时修改配置文件和实体，如果对不上的话程序启动不起来……

## 读取文件内容
```rust
pub fn load_config() -> ServerConfig {
    let current_path = env!("CARGO_MANIFEST_DIR");
    println!("current path: {}", current_path);
    // 读取配置文件
    let toml_file = format!("{}/configs/config.toml", current_path);

    let content = fs::read_to_string(toml_file).unwrap();
    let config: ServerConfig = toml::from_str(&content).unwrap();
    println!("config :{:?}", config);
    config
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::load_config;

    #[test]
    fn test() {
        load_config();
    }
}
```

load_config() 函数负责读取配置文件并解析为 ServerConfig 结构体。它使用了 fs::read_to_string() 函数来读取文件内容，并使用 toml::from_str() 函数将 TOML 格式的内容解析为结构体对象。

`env!("CARGO_MANIFEST_DIR")` 是 Rust 中的一个宏，用于获取当前 crate（即当前包）的根目录路径。在 Rust 中，每个 crate 都有一个与之关联的根目录，其中包含了 crate 的源代码、配置文件和其他相关文件。

`CARGO_MANIFEST_DIR` 是一个环境变量，由 Cargo 构建系统在构建过程中设置。它指示了当前 crate 的根目录路径。当使用 `env!("CARGO_MANIFEST_DIR")` 宏时，它会在编译时被展开为当前 crate 的根目录路径的字符串。

在上述代码中，`env!("CARGO_MANIFEST_DIR")` 用于获取当前 crate 的根目录路径，并结合其他路径信息构建配置文件的完整路径。这样可以确保代码在不同环境下都能正确地找到配置文件并加载配置信息。

## lazy_static
使用 lazy_static 宏，确保配置文件只会加载一次，避免了重复加载配置文件的开销。

```rust
lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = load_config();
}
```
lazy_static 块中定义的变量 SERVER_CONFIG，其访问属性为 pub，允许外部文件访问。
相当于一个全局变量，在其他文件中使用非常方便，例如：`let database_url = config::SERVER_CONFIG.database_url();`

main.rs
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = config::SERVER_CONFIG.database_url();
    let (host,port) = config::SERVER_CONFIG.get_app_host_port();
    db::init_db(database_url).await;

    println!("app started http://{}:{}",host,port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
            .service(api::user::routes())
    })
    .bind((host,port))?
    .run()
    .await
}
```

本文代码在 [github](https://github.com/hitolz/rust-web/tree/config)。











