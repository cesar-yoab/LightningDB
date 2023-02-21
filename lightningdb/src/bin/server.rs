use lightningdb::{server, DEFAULT_PORT};

use clap::Parser;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> lightningdb::Result<()> {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);

    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    name = "lightningdb-server",
    version,
    author,
    about = "A in-memory redis-like database server"
)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}
