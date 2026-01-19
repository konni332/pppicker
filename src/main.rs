use pppicker::run;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    let log_dir = dirs_next::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("pppicker");
    std::fs::create_dir_all(&log_dir)?;

    let file_appender = rolling::daily(log_dir, "pppicker.log");

    let file_layer = fmt::layer().with_writer(file_appender).with_ansi(false);

    let sterr_layer = fmt::layer().with_writer(std::io::stderr).with_ansi(true);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(sterr_layer)
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting pppicker");
    run()?;
    Ok(())
}
