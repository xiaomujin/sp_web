use time::macros::format_description;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub fn init_log() -> WorkerGuard {
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily("./logs", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let timer = LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3] +[offset_hour]:[offset_minute]"));
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_timer(timer);

    let std_writer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout) // 写入标准输出
        .with_ansi(true) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format.clone())
        .with_filter(LevelFilter::DEBUG);

    let file_writer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(std_writer)
        .with(file_writer)
        .init();

    tracing::debug!("Tracing initialized.");
    _guard
}
