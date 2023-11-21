use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    time,
};

use crate::dbtraits::DatabaseClient;
use crate::web3_client::Web3Client;
use crate::{constants::*, models::TokenInfo};
use log::info;

use web3::{types::{Log, H256, Address}, contract};
use web3::Result as Web3Result;

pub struct TokenSyncService {
    web3_client: Web3Client, // Assuming Web3Client is defined elsewhere
    db_client: Arc<dyn DatabaseClient + 'static>, 
    }

impl TokenSyncService {
    pub fn new(web3_client: Web3Client, db_client: Arc<dyn DatabaseClient>) -> Self {
        TokenSyncService {
            web3_client,
            db_client,
        }
    }
    pub async fn sync(self: Arc<Self>) -> web3::Result<()> {
        let self_clone = self.clone();
        let db_client_clone = self.db_client.clone();
    
        self.web3_client.subscribe_logs(
            None, // Optional custom topics
            Some(move |log| {
                let self_clone_inner = Arc::clone(&self_clone);
                // Spawn a new task for processing each log
                tokio::spawn(async move {
                    self_clone_inner.process_log(log).await;
                });
            }),
        ).await
    }

    // Process log function
    async fn process_log(&self, log: Log) {
        
        // info!("----------------------------------------");

        // info!("Block Number: {:?}", log.block_number.unwrap());
        // info!("block hash: {:?}", log.block_hash.unwrap());
        // info!("Transaction Index: {:?}", log.transaction_index.unwrap());

        // info!("Transaction Hash: {:?}", log.transaction_hash.unwrap());
        // info!("Log Index: {:?}", log.log_index.unwrap());
        // info!("log address: {:?}", log.address);
        // info!("From Address: {:?}", log.topics[1]);
        // info!("To Address: {:?}", log.topics[2]);
        // info!("Data: {:?}", format!("0x{}", hex::encode(log.data.0)));

        let mut token_info = TokenInfo {
            ..Default::default()
        };

        //Match the topic to determine the event type
        if let Some(first_topic) = log.topics.first() {
            info!("First Topic: {:?}", first_topic);
            match first_topic {
                topic if topic == &H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap() => {
                    // Handle V2 Pair Created event

                    let pair_result = self.parse_new_token(&log);
                    if let Ok((contract_address, token0_address, token1_address)) = pair_result {
                        token_info.contract_address = contract_address;
                    }
                }
                topic if topic == &H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap() => {
                    // Handle V3 New Token event

                    
                    let pair_result = self.parse_new_token(&log);
                    if let Ok((contract_address, token0_address, token1_address)) = pair_result {
                        token_info.contract_address = contract_address;
                    }
                }
                topic if topic == &H256::from_str(TOPIC_OWNER_TRANSFER).unwrap() => {
                    // Handle Owner Transfer event

                    let transfer_result = self.parse_owner_transfer(&log);

                    if let Ok((contract_address, from_address, to_address, is_renounced)) =
                        transfer_result
                    {
                        token_info.contract_address = contract_address;
                        token_info.is_renounced = is_renounced;
                    }
                }
                _ => {
                    // Handle unknown event
                    info!("Unknown event type");
                }

            }

            if !token_info.contract_address.is_empty() {
                let result = self.save_token(&mut token_info).await;
                if let Err(e) = result {
                    info!("Error saving token info: {}", e);
                }
            }    
        
            
        } else {
            info!("No topics in the log");
        }

        //info!("Event Address: {:?}", log.topics[0]);

        // info!("----------------------------------------");
        // info!("*****");
        // info!("*****");
       
    }

    pub async fn save_token(&self,  token_info: &mut TokenInfo) -> Result<bool, Box<dyn std::error::Error>> {

        let db = &self.db_client;
        
        
        let contract_address = Address::from_str(&token_info.contract_address);

        if let Err(e) = contract_address {
            info!("Error parsing contract address: {}", e);
            return Ok(false);
        }

        let addr = contract_address.unwrap();

        if token_info.name.is_empty() || token_info.symbol.is_empty() {

            let (name, symbol, decimals, total_supply) =
            self.web3_client.query_contract(addr, None).await?;

            token_info.name = name;
            token_info.symbol = symbol;
            token_info.decimals = decimals as i32;
            token_info.total_supply = format!("{:?}",total_supply);
            token_info.date_created = chrono::Utc::now().naive_utc();
        }
        
        let result = db.save_token(token_info).await;
        if let Err(e) = result {
            info!("Error saving token info to database: {}", e);
            return Ok(false);
        }
        Ok(true)
    }
    
    pub fn parse_new_token(
        &self,
        log: &Log ) -> Result<(String, Option<String>, Option<String>), Box<dyn std::error::Error>> {

            let topic_addr = self.convert_topic_to_address(log.topics[2]);

            let contract_address = format!("{:?}", topic_addr);
            let result = (contract_address, None, None);
            Ok(result)
        }

    pub fn parse_owner_transfer(
        &self,
        log: &Log,
    ) -> Result<(String, Option<String>, Option<String>, bool), Box<dyn std::error::Error>> {
        let contract_address = format!("{:?}", log.address);
        let mut from_address = None;
        let mut is_renounced = false;

        if log.topics.len() > 1 {
            from_address = Some(format!("{:?}", log.topics[1]));
        }
        let mut to_address = None;
        if log.topics.len() > 2 {
            if log.topics[2] == H256::from_str(TOPIC_ZERO_ADDRESS).unwrap() {
                is_renounced = true;
            }

            to_address = Some(format!("{:?}", log.topics[2]));
        }

        let result = (contract_address, from_address, to_address, is_renounced);
        info!("Owner transfer Result: {:?}", result);
        Ok(result)
    }

    fn convert_topic_to_address(&self, hash: H256) -> Address {
        Address::from_slice(&hash.0[12..])
    }
    // Other methods related to token synchronization...
}
