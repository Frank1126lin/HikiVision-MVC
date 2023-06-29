use chrono::Local;
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F > %T%.3f"))
    }
}

pub fn logging(non_blocking: tracing_appender::non_blocking::NonBlocking) {
    // Configure a custom event formatter
    let format = tracing_subscriber::fmt::format()
        .with_level(true) // include levels in formatted output
        .with_target(true) // include targets
        .with_thread_names(true)
        .with_timer(LocalTimer)
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .event_format(format)
        .with_writer(std::io::stdout) // 写入标准输出
        .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .init();
}
