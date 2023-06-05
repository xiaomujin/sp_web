use time::macros::format_description;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::{LocalTime};

pub fn init_log() -> WorkerGuard {
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily("./logs", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let timer = LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3] +[offset_hour]:[offset_minute]"));
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        // .with_source_location(true)
        .with_timer(timer);

    // 初始化并设置日志格式(定制和筛选日志)
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_writer(std::io::stdout) // 写入标准输出
            // .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
            .with_ansi(true)  // 如果日志是写入文件，应将ansi的颜色输出功能关掉
            .event_format(format)
            .init();
        tracing::info!("Debug mode!");
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(Level::WARN)
            // .with_writer(std::io::stdout)
            .with_writer(non_blocking)
            .with_ansi(false)
            .event_format(format)
            .init();
        tracing::info!("Release mode!");
    }

    tracing::trace!("tracing-trace");
    tracing::debug!("tracing-debug");
    tracing::info!("tracing-info");
    tracing::warn!("tracing-warn");
    tracing::error!("tracing-error");
    _guard
}
