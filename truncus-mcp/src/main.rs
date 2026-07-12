mod server;

use rmcp::{transport::stdio, ServiceExt};
use truncus_core::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    let service = server::TruncusMcp::new(config).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
