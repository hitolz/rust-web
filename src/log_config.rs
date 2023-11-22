use chrono::Local;
use fern::Dispatch;

use crate::config;

pub fn init_log() {
    let log_info = config::SERVER_CONFIG.get_log_info();

    let log_level = match log_info.level.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Error,
    };

    let log_path = log_info.path.as_str();

    let _x = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path).unwrap())
        .apply();
}
