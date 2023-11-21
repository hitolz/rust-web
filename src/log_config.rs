use chrono::{FixedOffset, Utc};
use env_logger::Builder;
use std::io::Write;

pub fn init_log() {
    // env_logger::init();

    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            let timestamp = Utc::now().with_timezone(&FixedOffset::east(8 * 3600));
            writeln!(
                buf,
                "{} [{}] [{}] {}",
                timestamp.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap(),
                record.args()
            )
        })
        .init();
}
