use std::str::FromStr;
use chrono::format::format;
use definetrsnew::{constants::*, modles::TokenInfo, etherscan::EtherscanClient};

use futures::pin_mut;
use log::info;
use tokio::signal;
use std::time;
use web3::{
    futures::{self, StreamExt},
    types::{BlockNumber, Filter, FilterBuilder, Log, H160, H256, U64}, ethabi::Token, api::Eth,
};
use std::sync::{Arc, Mutex};

type PendingAddresses = Arc<Mutex<Vec<H160>>>;

async fn process_addresses(pending_addresses: PendingAddresses, client: &EtherscanClient) {
    let mut addresses_to_remove = Vec::new();

    {
        // Lock the shared state for reading
        let addresses = pending_addresses.lock().unwrap();
        
        for address in addresses.iter() {
            let address_str = format!("{:?}", address);
            info!("Processing address: {:?}", address_str);

            match get_token_info(address_str.as_str(), &client).await {
                Ok(token_info) => {
                    info!("Success: {:?}", token_info);
                    addresses_to_remove.push(*address);
                },
                Err(e) => log::error!("Error: {}", e),
            }
        }
    } // Lock is released here as the scope ends

    // Lock the shared state for writing
    let mut addresses = pending_addresses.lock().unwrap();
    for address in addresses_to_remove {
        if let Some(index) = addresses.iter().position(|&x| x == address) {
            addresses.remove(index);
        }
    }
}

const  api_key: &str = "D1ZAF93H82CVWHGZ37J7EKM3RKWAD3323W";
#[tokio::main]
async fn main() -> web3::Result<()> { 
    env_logger::init();

    subscribe_logs().await?;
    Ok(())
}

async fn subscribe_logs() -> web3::Result<()> {
    info!("subscribing to logs.");
    // Connect to an Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(
        INFURA_URL,
    )?);
   
    let client = EtherscanClient::new(api_key.to_string());
   
    info!("connected to web3.");
    let pending_addresses = Arc::new(Mutex::new(Vec::<H160>::new()));
    let shared_addresses = pending_addresses.clone();

    // Define the topics you are interested in
    let topics = vec![

        H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap(),
        H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap(),
        H256::from_str(TOPIC_OWNER_TRANSFER).unwrap()
        
        ];

    info!("created topics.");
    // Create a filter
    let filter = FilterBuilder::default()
        .topics(Some(topics), None, None, None)
        .build();

    info!("created filter.");
    // Install the filter
    let filter_req = web3.eth_filter().create_logs_filter(filter).await?;
    let filter_req_clone = filter_req.clone();
    let logs_stream = filter_req.stream(time::Duration::from_secs(60));
    futures::pin_mut!(logs_stream);

    loop {
        let ctrl_c = signal::ctrl_c();
        tokio::select! {
            Some(result_log) = logs_stream.next() => {
                info!("log received.");
                match result_log {
                    Ok(log) => process_log(log, &client, pending_addresses.clone()).await,
                    Err(e) => {
                        info!("Error fetching log: {}", e);
                        continue;
                    }
                }
            }
            _ = ctrl_c => {
                info!("SIGINT received, exiting...");
                break;
            }
        }
    }

    // while let Some(result_log) = logs_stream.next().await {
    //     println!("log received.");
    //     let log: Log = match result_log {
    //         Ok(log) => log,
    //         Err(e) => {
    //             println!("{}", e);
    //             continue;
    //         }
    //     };

    //     let log_block_number = match log.block_number {
    //         Some(log_block_number) => log_block_number,
    //         None => {
    //             println!("log has no block number");
    //             continue;
    //         }
    //     };
    //     println!("event block number: {}", log.block_number.unwrap());
    //     process_log(log).await;
    // }

    info!("uninstalling filter.");
    let uninstall_result = filter_req_clone.uninstall().await?;
    info!("uninstalled filter: {:?}", uninstall_result);
    
    Ok(())
}

async fn process_log(log: Log, client: &EtherscanClient, pending_addresses: PendingAddresses) {
    info!("----------------------------------------");
    
    info!("Block Number: {:?}", log.block_number.unwrap());
    info!("block hash: {:?}", log.block_hash.unwrap());
    info!("Transaction Index: {:?}", log.transaction_index.unwrap());

    info!("Transaction Hash: {:?}", log.transaction_hash.unwrap());
    info!("Log Index: {:?}", log.log_index.unwrap());
    info!("log address: {:?}", log.address);
    
    //Match the topic to determine the event type
    if let Some(first_topic) = log.topics.first() {
        info!("First Topic: {:?}", first_topic);
        match first_topic {
            topic if topic == &H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap() => {
                // Handle V2 Pair Created event
             
                process_pair_created(&log, &client);
            },
            topic if topic == &H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap() => {
                // Handle V3 New Token event
                process_new_token(&log, &client).await;
            },
            topic if topic == &H256::from_str(TOPIC_OWNER_TRANSFER).unwrap() => {
                // Handle Owner Transfer event
                process_owner_transfer(&log, &client).await;

                pending_addresses.lock().unwrap().push(log.address);
            },
            _ => {
                // Handle unknown event
                info!("Unknown event type");
            }
        }
    } else {
        info!("No topics in the log");
    }

    //info!("Event Address: {:?}", log.topics[0]);
    info!("From Address: {:?}", log.topics[1]);
    info!("To Address: {:?}", log.topics[2]);
    info!("Data: {:?}", format!("0x{}", hex::encode(log.data.0)));
    info!("----------------------------------------");
    info!("*****");
    info!("*****");
    info!("Calling process_addresses");
    process_addresses(pending_addresses.clone(), &client).await;
}

async fn process_pair_created(log:&Log, client: &EtherscanClient){
    info!("V2 Pair Created Event");
}

async fn process_new_token(log: &Log, client: &EtherscanClient){

    info!("V3 New Token Event");

}

async fn process_owner_transfer(log: &Log, client: &EtherscanClient){
    info!("Owner Transfer Event");
  

    // let address_str = format!("{:?}", log.address);
    // info!("**** Address: {:?}", address_str.as_str());
    // match get_token_info(address_str.as_str(), &client).await {
        
    //     Ok(token_info) => {
    //         info!("Token Info: {:?}", token_info);
    //     },
    //     Err(e) => log::error!("Error occurred: {}", e),
    // }

}

async fn get_token_info(address: &str, client: &EtherscanClient) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    info!("Getting token info for address: {:?}", address);
    let response = client.get_token_info(address).await?;
    let mut token_info = TokenInfo::new(address);

    token_info.name = response.token_name;
    token_info.symbol = response.token_symbol;
    token_info.decimals = response.token_decimal;

    Ok(token_info)
}