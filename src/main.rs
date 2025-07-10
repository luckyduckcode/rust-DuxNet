mod core;
mod network;
mod api;
mod wallet;
mod frontend;

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting DuxNet Decentralized P2P Platform");
    
    // Create and start the DuxNet node
    let mut node = core::DuxNetNode::new(8080).await?;
    
    // Start the web API server
    let api_handle = tokio::spawn(async move {
        if let Err(e) = api::start_api_server(8081).await {
            error!("API server error: {}", e);
        }
    });
    
    // Start the P2P network
    let network_handle = tokio::spawn(async move {
        if let Err(e) = node.start().await {
            error!("Network error: {}", e);
        }
    });
    
    info!("DuxNet node started successfully!");
    info!("Web API available at: http://localhost:8081");
    info!("P2P node listening on port: 8080");
    
    // Wait for both handles
    tokio::try_join!(api_handle, network_handle)?;
    
    Ok(())
} 