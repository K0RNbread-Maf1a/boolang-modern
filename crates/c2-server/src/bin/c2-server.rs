use c2_server::{C2Config, C2Server};
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "c2-server")]
#[command(about = "Command and Control Server with SOCKS5 support", long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "configs/c2-server.toml")]
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
        let config = C2Config::default();
        config.save(&args.config)?;
        println!("✓ Generated default configuration at: {:?}", args.config);
        println!("⚠ IMPORTANT: Edit the configuration file and change:");
        println!("  - security.auth_token");
        println!("  - security.encryption_key");
        return Ok(());
    }

    // Load configuration
    let config = if args.config.exists() {
        C2Config::from_file(&args.config)?
    } else {
        println!("Config file not found, using defaults");
        C2Config::default()
    };

    println!("\n╔═══════════════════════════════════════╗");
    println!("║         C2 Server Starting            ║");
    println!("╚═══════════════════════════════════════╝\n");

    // Create and start server
    let mut server = C2Server::new(config);
    server.start().await?;

    Ok(())
}
