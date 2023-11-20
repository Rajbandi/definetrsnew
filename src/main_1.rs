use std::str::FromStr;


use futures::pin_mut;
use tokio::signal;
use std::time;
use web3::{
    futures::{self, StreamExt},
    types::{BlockNumber, Filter, FilterBuilder, Log, H160, H256, U64},
};

const INFURA_WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/fce68e74246240d3896b8f17081321ef";
const INFURA_URL: &str = "https://mainnet.infura.io/v3/fce68e74246240d3896b8f17081321ef";

const TOPIC_V2_PAIR_CREATED: &str = "0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9";

const TOPIC_V3_NEW_TOKEN: &str = "0xe1cf7aada88886bb170a3d9ac4236616de96205174f63e5506fdd6e24582d836";

const TOPIC_OWNER_TRANSFER: &str = "0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0";
const TOPIC_DEPOSIT: &str = "0xe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c";
const TOPIC_MINT: &str = "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f";
const TOPIC_SYNC: &str = "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1";
const TOPIC_SWAP: &str = "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822";
const TOPIC_MAX_LIMIT: &str = "0x947f344d56e1e8c70dc492fb94c4ddddd490c016aab685f5e7e47b2e85cb44cf";
const TOPIC_APPROVAL: &str = "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925";

const TOPIC_TRANSFER: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const TOPIC_ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

#[tokio::main]
async fn main() -> web3::Result<()> { 
    subscribe_logs().await?;
    Ok(())
}

async fn subscribe_logs() -> web3::Result<()> {
    println!("subscribing to logs.");
    // Connect to an Ethereum node
    let web3 = web3::Web3::new(web3::transports::Http::new(
        INFURA_URL,
    )?);

    println!("connected to web3.");
    // Define the topics you are interested in
    let topics = vec![

        H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap(),
        H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap(),
        H256::from_str(TOPIC_OWNER_TRANSFER).unwrap()
        
        ];

    println!("created topics.");
    // Create a filter
    let filter = FilterBuilder::default()
        .topics(Some(topics), None, None, None)
        .build();

    println!("created filter.");
    // Install the filter
    let filter_req = web3.eth_filter().create_logs_filter(filter).await?;
    let filter_req_clone = filter_req.clone();
    let logs_stream = filter_req.stream(time::Duration::from_secs(60));
    futures::pin_mut!(logs_stream);

    loop {
        let ctrl_c = signal::ctrl_c();
        tokio::select! {
            Some(result_log) = logs_stream.next() => {
                println!("log received.");
                match result_log {
                    Ok(log) => process_log(log).await,
                    Err(e) => {
                        println!("Error fetching log: {}", e);
                        continue;
                    }
                }
            }
            _ = ctrl_c => {
                println!("SIGINT received, exiting...");
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

    println!("uninstalling filter.");
    let uninstall_result = filter_req_clone.uninstall().await?;
    println!("uninstalled filter: {:?}", uninstall_result);
    
    Ok(())
}

async fn process_log(log: Log) {
    println!("----------------------------------------");
    
    println!("Block Number: {:?}", log.block_number.unwrap());
    println!("block hash: {:?}", log.block_hash.unwrap());
    println!("Transaction Index: {:?}", log.transaction_index.unwrap());

    println!("Transaction Hash: {:?}", log.transaction_hash.unwrap());
    println!("Log Index: {:?}", log.log_index.unwrap());
    println!("log address: {:?}", log.address);
    
    //Match the topic to determine the event type
    if let Some(first_topic) = log.topics.first() {
        println!("First Topic: {:?}", first_topic);
        match first_topic {
            topic if topic == &H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap() => {
                // Handle V2 Pair Created event
                println!("V2 Pair Created Event");
            },
            topic if topic == &H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap() => {
                // Handle V3 New Token event
                println!("V3 New Token Event");
            },
            topic if topic == &H256::from_str(TOPIC_OWNER_TRANSFER).unwrap() => {
                // Handle Owner Transfer event
                println!("Owner Transfer Event");
            },
            _ => {
                // Handle unknown event
                println!("Unknown event type");
            }
        }
    } else {
        println!("No topics in the log");
    }

    //println!("Event Address: {:?}", log.topics[0]);
    println!("From Address: {:?}", log.topics[1]);
    println!("To Address: {:?}", log.topics[2]);
    println!("Data: {:?}", format!("0x{}", hex::encode(log.data.0)));
    println!("----------------------------------------");
    println!("*****");
    println!("*****");
}

async fn process_new_token(log: Log){



}

async fn process_owner_transfer(log: Log){


}

async fn retrieve_token_info(log: Log){
 
}