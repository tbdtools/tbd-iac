use anyhow::Result;
use tbd_iac::cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::handle_cli().await
}
