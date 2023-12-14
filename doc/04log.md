# Rust web 开发-4.log

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
本篇介绍一下日志模块。


## 日志门面 log
日志门面不是说排场很大的意思，而是指相应的日志 API 已成为事实上的标准，会被其他日志框架使用。通过这种统一的门面，开发者就可以不必拘泥于日志框架的选择，未来大不了再换就是。

>A Rust library providing a lightweight logging facade.

官方库 [log](https://github.com/rust-lang/log)。

提供了日志输出的宏： trace!、debug!、info!、warn!、error!输出对应级别的日志内容。

官方库只是提供了一个接口，使用的时候还需要有具体的实现库。

一些日志库：
### 简单小型的日志库
env_logger
simple_logger
simplelog
pretty_env_logger
stderrlog
flexi_logger
call_logger
std-logger
structured-logger

### 复杂的可配置框架
log4rs
fern

### WebAssembly 库
console_log


本文选择其中几个库，介绍一下其用法。 介绍 env_logger 还有 《Rust 语言圣经》里介绍的 tracing。

## env_logger

### 添加依赖
```toml
log = "0.4"
env_logger = "0.10"
```

### 编写日志 init 方法
```rust
fn init_log()  {
    env_logger::init();
}
```
在 main 方法第一行调用日志初始化方法，然后输出日志  `info!("app started http://{}:{}",host,port);`

```rust
async fn main() -> std::io::Result<()> {
    init_log();

    let database_url = config::SERVER_CONFIG.database_url();
    let (host, port) = config::SERVER_CONFIG.get_app_host_port();
    db::init_db(database_url).await;

    info!("app started http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
            .service(api::user::routes())
    })
    .bind((host, port))?
    .run()
    .await
}
```
要注意的一点是，启动命令不再是 `cargo run`，而是要指定一个环境变量 `RUST_LOG=info`。完整的启动命令是`RUST_LOG=info cargo run`，启动后就能够看到日志输出到控制台了。

```
[2023-11-21T07:53:56Z INFO  rust_web] app started http://127.0.0.1:8099
[2023-11-21T07:53:56Z INFO  actix_server::builder] starting 10 workers
[2023-11-21T07:53:56Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
```

### 启动命令修改
如果不想每次启动时都输入这么长的命令，`RUST_LOG=info cargo run`，可以使用上一篇 config 中没有介绍的 .env 配置文件，将环境变量放在 .env 文件里。
在项目根目录创建 .env 文件然后填写
```
RUST_LOG=info
```

然后添加依赖
```toml
dotenv = "0.15.0"
```

main 方法启动第一行加载环境变量
```rust
use dotenv::dotenv;

dotenv().ok();
```

这时候，直接执行 `cargo run` 启动即可。

### 修改日期格式
可以看到，输出的日志日期格式是这样的`[2023-11-21T07:53:56Z INFO `，

```rust
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
```
这时输出的日志是这样的:`2023-11-21 17:14:50 [INFO] [rust_web] app started http://127.0.0.1:8099`

### 输出日志到文件

```rust
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
```
env_logger 目前看来只能配置一个 target，输出到文件之后控制台就看不到日志输出了,


env_logger 篇代码在 [github](https://github.com/hitolz/rust-web/tree/log_env_logger)。

接下来介绍一下 tracing。
## tracing
tracing 演示使用的还是0.1 版本，0.2 版本不是正式版本。

tracing 并不是一个日志库，而是一个分布式跟踪的 SDK，用来采集监控数据的。但是它也支持 log 门面库 API，所以可以当做日志库来使用。

###  span、event、Collector
tracing 中最重要的三个核心概念 span、event、Collector收集器。

#### span
span 最大的意义在于它可以记录一个过程，也就是在一段时间内发生的事件流。有开始和结束。

```rust
fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); // enter 后进入该 span 的上下文
    info!("hello from span ")
} // 离开作用域后，_enter 被 drop，对应的 span 在此结束
```

输出内容：`2023-11-22 10:40:41.669  INFO span_for_test: rust_tracing: hello from span`

使用`#[instrument]` 创建 span。
当使用了`#[instrument]` ，tracing 会为以后函数主动创立 span ，该 span 名与函数雷同，并且整个函数都在该 span 的上下文内。
```rust
#[instrument]
fn expensive_work(secs: u64) {
    info!("doing expensive work");
    sleep(Duration::from_secs(secs));
    info!("done with expensive work");
}

fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); // enter 后进入该 span 的上下文
    info!("hello from span ");
    expensive_work(2);
} // 离开作用域后，_enter 被 drop，对应的 span 在此结束

```
输出内容为
```log
2023-11-22 10:48:13.086  INFO span_for_test: rust_tracing: hello from span     
2023-11-22 10:48:13.086  INFO span_for_test:expensive_work{secs=2}: rust_tracing: doing expensive work    
2023-11-22 10:48:15.091  INFO span_for_test:expensive_work{secs=2}: rust_tracing: done with expensive work 
```

#### event
event 代表了某个时间点发生的事件，跟日志类似，不同的是 event 可以产生在 span 的上下文中。


```rust
fn main() {
    init_log(); // 使用 tracing_subscriber 初始化 Collector
    let span = span!(Level::INFO, "span_for_test");
    let _enter = span.enter(); 
    info!("hello from span ");
    event!(Level::INFO, "event hello from span ");
    expensive_work(2);
} 

--- 输出内容与 info 一样。
2023-11-22 10:49:47.585  INFO span_for_test: rust_tracing: hello from span     
2023-11-22 10:49:47.585  INFO span_for_test: rust_tracing: event hello from span 
```


#### Collector
当 span 或 event 发生时，会被实现了 Collect 特征的收集器所记录或聚合，这个过程是通过通知的方式实现的：当 event 发生或者 span 开始/结束时，会调用 Collect 特征的相应方法通知 Collector。

Collector 会将 span 和 event 以一定的格式输出到指定的地方，比如：stdout、stderr、文件、网络等。

tracing-subscriber 提供了 Collector，可以方便的输出事件信息。

### 添加依赖
```
[dependencies]
tracing = "0.1"
tracing-subscriber = {version = "0.3",features = ["env-filter", "time", "local-time", ] }
time = { version = "0.3.7", features = ["macros"] }
```

### 编写日志 init 方法
```rust
use time::UtcOffset;
use time::macros::format_description;
use tracing_subscriber::fmt::time::OffsetTime;

pub fn init_log() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt().with_timer(local_time).init();
}
```
输出日志格式为 `2023-11-21 21:05:54.559  INFO rust_web: app started http://127.0.0.1:8099  `

tracing 的时间格式化限制了必须使用 time 库中的时间格式。

### 输出日志到文件
tracing-appender = "0.1"

```rust
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
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(timer)
        // .with_timer(LocalTimer)
        .with_writer(stdout.and(appender))
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
}

```
tracing 输出到文件要用 tracing_appender

tracing_appender::rolling:: 提供了几种日志切割方式，
1. minutely
2. hourly
3. daily
4. never


## 小结
tracing 是一个强大的、分布式的日志和诊断框架。它提供了一套用于在Rust应用程序中进行结构化日志记录和性能分析的工具。tracing 的主要特点是它支持异步的、事件驱动的日志记录，并提供了灵活的上下文传播机制，使你能够在不同的线程或任务之间跟踪日志记录。tracing 还支持各种日志目标和格式，并提供了丰富的插件生态系统，可以扩展其功能。tracing 适合于需要更高级的日志记录和分析需求的项目。

env_logger 是一个简单易用的日志记录库，它使用环境变量来配置日志级别和输出格式。env_logger 适合于快速启动和调试，它不需要复杂的配置文件，而是依赖于环境变量来控制日志记录的行为。env_logger 提供了一些默认的日志格式和目标，但相对于 tracing，它的可定制性较低。env_logger适合于开发和调试阶段，但在生产环境中可能需要更高级的日志记录功能。


