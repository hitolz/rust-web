use std::fs;

use lazy_static::*;
use log::info;
use serde::Deserialize;
use crate::log_config;
use dotenv::dotenv;

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

impl ServerConfig {
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

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = load_config();
}

pub fn load_config() -> ServerConfig {

    dotenv().ok();
    log_config::init_log();


    let current_path = env!("CARGO_MANIFEST_DIR");
    info!("current path: {}", current_path);
    // 读取配置文件
    let toml_file = format!("{}/configs/config.toml", current_path);

    let content = fs::read_to_string(toml_file).unwrap();
    let config: ServerConfig = toml::from_str(&content).unwrap();
    info!("config :{:?}", config);
    config
}

#[cfg(test)]
mod tests {
    use super::load_config;

    #[test]
    fn test() {
        load_config();
    }
}
