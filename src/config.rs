use std::fs;

use lazy_static::*;
use log::info;
use serde::Deserialize;

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
pub struct Log {
    pub level: String,
    pub path: String,
}
#[derive(Deserialize, Default, Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub group_id: String,
}


#[derive(Deserialize, Default, Debug, Clone)]
pub struct ServerConfig {
    pub app: App,
    pub database: Database,
    pub log: Log,
    pub kafka_config: KafkaConfig,
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

    pub fn get_log_info(&self) -> &Log {
        &self.log
    }
}

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = load_config();
}

pub fn load_config() -> ServerConfig {
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
