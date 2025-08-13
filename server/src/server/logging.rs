use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() {
    let file_appender = tracing_appender::rolling::hourly("logs", "blobfishapp.log");
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("monolith=info,tower_http=info")),
        )
        .with(fmt::layer().with_file(true).with_line_number(true))
        .with(fmt::Layer::default().with_writer(file_appender).json())
        .init();
}
