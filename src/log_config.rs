use time::macros::{format_description, offset};
use tracing_subscriber::fmt::time::{FormatTime, OffsetTime};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::EnvFilter;

// 自定义时间格式化
struct LocalTimer;
const fn east8() -> Option<chrono::FixedOffset> {
    chrono::FixedOffset::east_opt(8 * 3600)
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&east8().unwrap());
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%SS"))
    }
}

pub fn init_log() {
    let time_fmt =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");
    let timer = OffsetTime::new(offset!(+8), time_fmt);

    let appender = tracing_appender::rolling::daily("log/", "app.log");
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_timer(timer)
        // .with_timer(LocalTimer)
        .with_writer(stdout.and(appender))
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
}
