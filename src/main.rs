use clap::Parser;
use signal_hook::consts::{SIGINT, SIGTERM};
use tracing::{error, info};

use velocity_engine::config::loader::ConfigLoader;
use velocity_engine::daemon::DaemonService;
use velocity_engine::error::Result;
use velocity_engine::logging;

#[derive(Parser)]
#[command(name = "velocity-engine")]
#[command(about = "Velocity Engine - Gaming optimization daemon", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/etc/velocity-engine/default.toml")]
    config: String,
    #[arg(short, long)]
    verbose: bool,
}

static SHUTDOWN: std::sync::LazyLock<std::sync::Arc<std::sync::atomic::AtomicBool>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)));

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        error!("Fatal error: {}", e);
        std::process::exit(1);
    }
}

async fn run(args: Args) -> Result<()> {
    let config = load_config(&args.config).await?;

    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }

    logging::init(&config.logging)?;
    info!("Starting Velocity Engine v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration loaded from {}", args.config);

    setup_signal_handlers();
    let mut service = DaemonService::new(config).await?;
    service.start().await?;

    if let Err(e) = service.run().await {
        error!("Daemon error: {}", e);
    }

    if let Err(e) = service.shutdown().await {
        error!("Shutdown error: {}", e);
    }

    info!("Velocity Engine stopped");
    Ok(())
}

async fn load_config(path: &str) -> Result<velocity_engine::config::types::Config> {
    let loader = ConfigLoader::new(path).await?;
    let config = loader.get().await.clone();
    Ok(config)
}

fn setup_signal_handlers() {
    for sig in [SIGINT, SIGTERM] {
        let _ = signal_hook::flag::register(sig, SHUTDOWN.clone());
    }
}
