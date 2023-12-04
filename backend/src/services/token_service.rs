use std::{str::FromStr, sync::Arc};

use crate::clients::{DatabaseClient, EtherscanClient};
use crate::clients::Web3Client;
use crate::models::TokenQuery;
use crate::services::WebSocketService;
use crate::utils::TokenUtil;
use crate::{constants::*, models::TokenInfo};

use log::info;

use serde_json::{json, Value};
use tokio::sync::RwLock;


use web3::types::{Address, Log, H256, U256};

pub struct TokenService {
    web3_client: Web3Client, // Assuming Web3Client is defined elsewhere
    db_client: Arc<dyn DatabaseClient + 'static>,
    etherscan_client: EtherscanClient,
    latest_tokens: RwLock<Vec<TokenInfo>>,
}

impl TokenService {
    pub fn new(web3_client: Web3Client, db_client: Arc<dyn DatabaseClient>, etherscan_client: EtherscanClient) -> Self {
        TokenService {
            web3_client,
            db_client,
            etherscan_client,
            latest_tokens: RwLock::new(Vec::new()),
        }
    }
    pub async fn sync(self: Arc<Self>) -> web3::Result<()> {
        let self_clone = self.clone();

        self.web3_client
            .subscribe_logs(
                None, // Optional custom topics
                Some(move |log| {
                    let self_clone_inner = Arc::clone(&self_clone);
                    // Spawn a new task for processing each log
                    tokio::spawn(async move {
                        self_clone_inner.process_log(log).await;
                    });
                }),
            )
            .await
    }

    // Process log function
    async fn process_log(&self, log: Log) {
        info!("Processing log...");

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
                        token_info.is_v3 = false;
                        
                    }
                }
                topic if topic == &H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap() => {
                    // Handle V3 New Token event

                    let pair_result = self.parse_new_token(&log);
                    if let Ok((contract_address, token0_address, token1_address)) = pair_result {
                        token_info.contract_address = contract_address;
                        token_info.is_v3 = true;
                       
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
                        token_info.owner = to_address.clone();
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

    pub async fn save_token(
        &self,
        token_info: &mut TokenInfo,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let db = &self.db_client;

        let contract_address = Address::from_str(&token_info.contract_address);

        if let Err(e) = contract_address {
            info!("Error parsing contract address: {}", e);
            return Ok(false);
        }

        let addr = contract_address.unwrap();

        if token_info.name.is_empty() || token_info.symbol.is_empty() {
            let (name, symbol, decimals, total_supply, code) =
                self.web3_client.query_contract(addr, None).await?;

            if total_supply > U256::from(0) {
                let code_clone = code.clone();
                token_info.name = name;
                token_info.symbol = symbol;
                token_info.decimals = decimals as i32;
                token_info.total_supply = format!("{:?}", total_supply);
                token_info.date_created = chrono::Utc::now().naive_utc();

                if code_clone.is_some() {
                    let code_str = code.clone().unwrap();
                    let code_hex = TokenUtil::extract_hex_string(code_str.as_str());
                    if code_hex.is_some() {
                        token_info.code = Some(code_hex.unwrap());
                    } else {
                        token_info.code = Some(code.unwrap());
                    }
                }
               // token_info.is_verified = !code_clone.is_none() && !code_clone.unwrap().is_empty()
            } else {
                info!("Total supply is zero************");
                return Ok(false);
            }
        }

        
        if !token_info.name.is_empty() && !token_info.contract_address.is_empty() {
            if !token_info.is_verified {
                let addr_str = format!("{:?}", addr);
                info!("*************** Getting token verified for address: {:?}", addr_str);
                let abi_response = self.etherscan_client.get_token_verified(addr_str.clone()).await;
                match abi_response {
                    Ok(abi_response) => {
                        info!(" Token verified: {:?}", abi_response.status);
                        if abi_response.status == "1" {
                            token_info.is_verified = true;
                            if abi_response.result.is_empty() {
                                info!("No ABI found for address: {:?}", addr_str);
                            }
                            else {
                                info!("Parsing abi result");
                                let abi:  Result<Value, serde_json::Error> = serde_json::from_str(&abi_response.result);
                                match abi {
                                    Ok(parsed_json) => {
                                        info!("ABI: {:?}", parsed_json);
                                        token_info.abi = Some(parsed_json.to_string());
                                    },
                                    Err(e) => info!("Failed to parse JSON: {:?}", e),
                                }
                            }
                        }
                    }
                    Err(e) => info!("Error getting token verified: {}", e),
                }
            }
        }

        let result = db.save_token(token_info).await;
        if let Err(e) = result {
            info!("Error saving token info to database: {}", e);
            return Ok(false);
        }
        let message = serde_json::json!({
            "type": "tokenupdate",
            "data": token_info
        });

        WebSocketService::send_to_general_clients(&message.to_string()).await;
        // Update in-memory storage
        self.update_memory_storage(token_info.clone()).await?;
      
        Ok(true)
    }
    pub async fn update_memory_storage(&self, token_info: TokenInfo) -> Result<(), Box<dyn std::error::Error>> {
        info!("************* Updating in-memory storage ***************");
        let mut latest_tokens = self.latest_tokens.write().await;
        
        if let Some(index) = latest_tokens.iter().position(|t| t.contract_address == token_info.contract_address) {
            info!("Token exists, updating it");
            // Token exists, update it
            latest_tokens[index] = token_info;
        } else {
            info!("Token does not exist, insert at the beginning");
            // Token does not exist, insert at the beginning
            latest_tokens.insert(0, token_info);
    
            // Ensure we don't exceed 100 tokens
            info!("Checking if latest tokens exceed 100");
            if latest_tokens.len() > 100 {
                info!("Latest tokens exceed 100, removing the oldest token");
                latest_tokens.pop(); // Remove the oldest token
            }
        }
    
        info!("************* In-memory storage updated **********");
        Ok(())
    }
    
    pub fn parse_new_token(
        &self,
        log: &Log,
    ) -> Result<(String, Option<String>, Option<String>), Box<dyn std::error::Error>> {
        let mut contract_address = format!("{:?}", log.address);
        let topic_addr1 = self.convert_topic_to_address(log.topics[1]);
        let topic_addr2 = self.convert_topic_to_address(log.topics[2]);

        let topic_addr1_str = format!("{:?}", topic_addr1);
        let topic_addr2_str = format!("{:?}", topic_addr2);

        //now search topic_addr1 in the list TOKEN_WETH, TOKEN_USDC, TOKEN_USDT
        //if found, contract_address is topic_addr2, write code below
        if topic_addr1_str == TOKEN_WETH
            || topic_addr1_str == TOKEN_USDC
            || topic_addr1_str == TOKEN_USDT
        {
            contract_address = topic_addr2_str.clone();
        } else if topic_addr2_str == TOKEN_WETH
            || topic_addr2_str == TOKEN_USDC
            || topic_addr2_str == TOKEN_USDT
        {
            contract_address = topic_addr1_str.clone();
        } else {
            info!("No WETH, USDC, USDT found");
        }

        let result = (
            contract_address,
            Some(topic_addr1_str),
            Some(topic_addr2_str),
        );
        info!("Parse new token result: {:?}", result);
        return Ok(result);
    }

    pub fn parse_owner_transfer(
        &self,
        log: &Log,
    ) -> Result<(String, Option<String>, Option<String>, bool), Box<dyn std::error::Error>> {
        let contract_address = format!("{:?}", log.address);
        let mut from_address = None;
        let mut to_address = None;
        let mut is_renounced = false;

        if log.topics.len() > 2 {
            let topic_addr1 = self.convert_topic_to_address(log.topics[1]);
            let topic_addr2 = self.convert_topic_to_address(log.topics[2]);

            let topic_addr1_str = format!("{:?}", topic_addr1);
            let topic_addr2_str = format!("{:?}", topic_addr2);

            from_address = Some(topic_addr1_str.clone());
            to_address = Some(topic_addr2_str.clone());

            info!("Checking if to_address is zero address {:?}", to_address);
            if to_address == Some(ZERO_ADDRESS.to_string()) {
                is_renounced = true;
            }
        }

        let result = (contract_address, from_address, to_address, is_renounced);
        info!("Owner transfer Result: {:?}", result);
        Ok(result)
    }

    pub async fn get_token(
        &self,
        contract_address: &str,
    ) -> Result<TokenInfo, Box<dyn std::error::Error>> {
        self.db_client
            .get_token(contract_address)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_tokens(
        &self,
        query: TokenQuery,
    ) -> Result<Vec<TokenInfo>, Box<dyn std::error::Error>> {
        self.db_client
            .get_all_tokens(&query)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_latest_tokens(&self) -> Result<(), Box<dyn std::error::Error>> {
        let query = TokenQuery { 
            name: None, 
            symbol: None, 
            contract_address: None, 
            is_verified: None, 
            is_renounced: None, 
            is_active: None, 
            from_date: None, 
            to_date: None, 
            sort_by: Some("date_created_desc".to_string()), 
            limit: Some(100), // Example: limit to 100 tokens
            offset: Some(0) 
        };
        info!("********** Getting all tokens from database");
        let tokens = self.db_client.get_all_tokens(&query).await?;
        info!("********** Got all tokens from database {}", tokens.len());
        // Properly await the future returned by write()
        info!("********** Updating latest tokens in memory");
        let mut latest_tokens = self.latest_tokens.write().await;
        *latest_tokens = tokens;
        info!("********** Updated latest tokens in memory");
        Ok(())
    }
    
    pub async fn get_latest_tokens(&self) -> Result<Vec<TokenInfo>, Box<dyn std::error::Error>> {
        // First, try to read from the in-memory storage.
        {
            info!("********** Readiing latest tokens tokens");
            let latest_tokens = self.latest_tokens.read().await;
            if !latest_tokens.is_empty() {
                info!("Latest tokens found in memory");
                return Ok(latest_tokens.clone());
            }
        }
        info!("Latest tokens not found in memory");

        // If the in-memory storage is empty, update it from the database.
        self.update_latest_tokens().await?;
        info!("Latest tokens updated from database");

        info!("Now reading latest tokens from memory");
        // Read and return the latest tokens from the updated in-memory storage.
        let latest_tokens = self.latest_tokens.read().await;
        info!("Latest tokens read from memory ");

        Ok(latest_tokens.clone())

    }
    

    fn convert_topic_to_address(&self, hash: H256) -> Address {
        Address::from_slice(&hash.0[12..])
    }
    // Other methods related to token synchronization...
}
