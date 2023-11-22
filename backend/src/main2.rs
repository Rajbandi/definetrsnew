use ethereum_types::{Bloom, BloomInput, H256, U64};
use web3::{Web3, transports::WebSocket};
use futures::stream::StreamExt;
use std::sync::Arc;
use log::{info, error};
use std::str::FromStr;
use std::time::Duration;
use hex;

//Declare hex string as constant
const PAIR_WETH: &str = "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
const PAIR_USDC: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

const TOPIC_V2_PAIR_CREATED: &str = "0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9";

//convert TOPIC_V2_PAIR_CREATED to H256




const TRANSFER_TOPIC: H256 = H256([
    0xdd, 0xf2, 0x52, 0xad, 0x1b, 0xe2, 0xc8, 0x9b, 
    0x69, 0xc2, 0xb0, 0x68, 0xfc, 0x37, 0x8d, 0xaa, 
    0x95, 0x2b, 0xa7, 0xf1, 0x63, 0xc4, 0xa1, 0x16, 
    0x28, 0xf5, 0x5a, 0x4d, 0xf5, 0x23, 0xb3, 0xef,
]);
// Function to check if bloom filter contains the topic
fn contains_topic(bloom: &Bloom, topic: &H256) -> bool {
    
    //rewrite the below code to use the bloom filter crate
   
   //check bloom contains topic , provide statement below

    log::info!("Bloom contains topic: {:?}", bloom.contains_input(BloomInput::Raw(&topic.0)));
    bloom.contains_input(BloomInput::Raw(&topic.0))
}

// Subscribe function
async fn subscribe_to_new_block_headers_and_check_bloom(
    web3_shared: Arc<Web3<WebSocket>>,
    topics: Vec<H256>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut block_stream = web3_shared.eth_subscribe().subscribe_new_heads().await?;

    while let Some(block_header) = block_stream.next().await {
        let block_header = block_header?;
   
        info!("New block: {:?}", block_header);
        let bloom = Bloom::from_slice(&block_header.logs_bloom.0);
        if topics.iter().any(|topic| contains_topic(&bloom, topic)) {
            info!("Block contains topic ");
            let web3_clone = web3_shared.clone();
            let block_number = block_header.number.unwrap_or_default();

            tokio::spawn(async move {
                if let Err(e) = process_block_logs(web3_clone, block_number).await {
                    error!("Error processing block logs: {:?}", e);
                }
            });
        }
    }

    Ok(())
}

async fn maintain_connection(web3_shared: Arc<Web3<WebSocket>>, topics: Vec<H256>) -> Result<(), Box<dyn std::error::Error>> {
    let mut error_count = 0;
    let error_limit = 5; // Maximum number of consecutive errors

    loop {
        let result = subscribe_to_new_block_headers_and_check_bloom(web3_shared.clone(), topics.clone()).await;

        match result {
            Ok(_) => error_count = 0, // Reset error count on success
            Err(e) => {
                error!("Error: {:?}", e);
                error_count += 1;
                if error_count >= error_limit {
                    error!("Reached maximum number of consecutive errors. Exiting...");
                    break;
                }
                error!("Attempting to reconnect...");
                tokio::time::sleep(Duration::from_secs(5)).await; // Wait before reconnecting
            }
        }
    }

    Ok(())
}

// Function to process the logs of a block
// async fn process_block_logs(_web3: Arc<Web3<WebSocket>>, block_number: U64) -> Result<(), Box<dyn std::error::Error>> {
//     // Your logic for processing the logs of a block
//     info!("Processing block number {:?}", block_number);
//     // ...
//     Ok(())
// }

async fn process_block_logs(web3: Arc<Web3<WebSocket>>, block_number: U64) -> Result<(), Box<dyn std::error::Error>> {
    let topics = vec![TRANSFER_TOPIC];

    let filter = web3::types::FilterBuilder::default()
    .from_block(web3::types::BlockNumber::Number(block_number))
    .to_block(web3::types::BlockNumber::Number(block_number))
    .topics(Some(topics), None, None, None)
    .build();

    let logs = web3.eth().logs(filter).await?;

    for log in logs.iter() {
        match log.topics.first() {
            Some(topic) if topic == &TRANSFER_TOPIC => {
                if log.topics.len() == 3 
                    && log.topics[1] == H256::default() // Check if 'from' address is zero
                {
                    info!("Transfer from zero address in block {}", block_number);
                    let to_address = log.topics[2];
                    let code = web3.eth().code(to_address.into(), None).await?;
                    info!("Code at address {:?}: {:?}", to_address, code);
                    let code_hex = format!("0x{}", hex::encode(code.0));

                    if code_hex != "0x"{
                        // This is a transfer to a contract address
                        info!("T********ransfer to contract address in block {}", block_number);
                    }
                    else{
                        info!("!!!!!!!!!Transfer to non-contract address in block {}", block_number);
                    }
                }
            },
            // ... Add handling for other topics
            _ => {}
        }
    }

    Ok(())
}

// public static readonly PAIR_WETH = "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
// public static readonly PAIR_USDC = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
// public static readonly PAIR_USDT = "";
// public static readonly PAIR_DAI = "";
// public static readonly PAIR_BUSD = "";
// public static readonly PAIR_WBTC = "";
// public static readonly PAIR_LINK = "";
// public static readonly PAIR_UNI = "";

// public static readonly TOPIC_V2_PAIR_CREATED = "0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9";
// public static readonly TOPIC_V3_NEW_TOKEN = "0xe1cf7aada88886bb170a3d9ac4236616de96205174f63e5506fdd6e24582d836";
// public static readonly TOPIC_OWNER_TRANSFER = "0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0";
// public static readonly TOPIC_TRANSFER = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
// public static readonly TOPIC_ZERO_ADDRESS = "0x0000000000000000000000000000000000000000000000000000000000000000";
// public static readonly TOPIC_APPROVAL = "";
// public static readonly TOPIC_BURN = "";
// public static readonly TOPIC_MINT = "";
// public static readonly TOPIC_SWAP = "";
// public static readonly TOPIC_ADD_LIQUIDITY = "";
// public static readonly TOPIC_REMOVE_LIQUIDITY = "";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    let binding = format!("{:x}", TRANSFER_TOPIC).clone();
    let topic_hex_strings = vec![binding.as_str()];

    let mut topics = Vec::new();
    for hex_str in topic_hex_strings {
        let topic = H256::from_str(hex_str)
            .map_err(|_| format!("Invalid hex string: {}", hex_str))?;
        topics.push(topic);
    }

    let infura_url =  "wss://mainnet.infura.io/ws/v3/fce68e74246240d3896b8f17081321ef"    ;
    let transport = WebSocket::new(infura_url).await?;
    let web3 = Web3::new(transport);
    let web3_shared = Arc::new(web3);

    maintain_connection(web3_shared, topics).await?;

    Ok(())
}
