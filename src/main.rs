mod presentation;
mod infrastructure;
mod usecase;
mod domain;

use presentation::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start().await?;
    Ok(())
}
