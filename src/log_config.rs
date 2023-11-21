use chrono::{FixedOffset, Utc};
use env_logger::{Builder, Target};
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_log() {
    // env_logger::init();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("app.log")
        .expect("Failed to open log file");
    let target = Box::new(file);

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
        .target(Target::Pipe(target))
        .init();
}
