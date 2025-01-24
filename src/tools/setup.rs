use tracing_appender::{non_blocking::{NonBlocking, WorkerGuard}, rolling};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};


pub fn setup_log() -> (NonBlocking, WorkerGuard, String) {
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "link_server=info,axum=error".to_owned());
    (non_blocking, guard, log_filter)
}