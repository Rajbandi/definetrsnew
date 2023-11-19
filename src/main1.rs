use web3::{Web3, transports::WebSocket};
use futures::stream::StreamExt; // Import StreamExt for stream handling

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    subscribe_to_new_block_headers().await?;
    Ok(())
}

async fn subscribe_to_new_block_headers() -> Result<(), Box<dyn std::error::Error>> {
    let infura_url = "wss://mainnet.infura.io/ws/v3/fce68e74246240d3896b8f17081321ef";
    let transport = WebSocket::new(infura_url).await?;
    let web3 = Web3::new(transport);
    let mut block_stream = web3.eth_subscribe().subscribe_new_heads().await?;

    while let Some(block) = block_stream.next().await {
        match block {
            Ok(block_header) => {
                println!("New block: {:?}", block_header);
            },
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
