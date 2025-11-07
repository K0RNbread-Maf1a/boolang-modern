use c2_server::{ReverseShellConfig, ReverseShellServer};
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "shell-server")]
#[command(about = "Reverse Shell Server with TLS and SOCKS5 support", long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "configs/reverse-shell.toml")]
    config: PathBuf,

    /// Generate default configuration and exit
    #[arg(long)]
    generate_config: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();

    if args.generate_config {
        let config = ReverseShellConfig::default();
        config.save(&args.config)?;
        println!("✓ Generated default configuration at: {:?}", args.config);
        return Ok(());
    }

    // Load configuration
    let config = if args.config.exists() {
        ReverseShellConfig::from_file(&args.config)?
    } else {
        println!("Config file not found, using defaults");
        ReverseShellConfig::default()
    };

    println!("\n╔═══════════════════════════════════════╗");
    println!("║    Reverse Shell Server Starting      ║");
    println!("╚═══════════════════════════════════════╝\n");

    // Create and start server
    let mut server = ReverseShellServer::new(config);
    server.start().await?;

    Ok(())
}
