use log::{info, error};
use futures::pin_mut;
use serde::de;
use tokio::signal;

use std::{
    str::FromStr,
    time
};
use crate::constants::*;
use web3::{
    Web3,
    error::Error as Web3Error,
    futures::{self, StreamExt},
    contract::{Contract, Options},
    transports::Http,
    types::{BlockNumber, Filter, FilterBuilder, Log, H160, H256, U64,Address, U256, Bytes}
};
pub struct Web3Client {
    web3: Web3<Http>,
    generic_abi: Bytes,
}

impl Web3Client {
    pub async fn new(api_url: &str) -> Result<Self, web3::Error> {
        info!("Connecting to web3 at: {}", api_url);
        let transport = Http::new(api_url)?;
        info!("Connected to web3 at: {}", api_url);

        info!("Loading generic ABI");
        let generic_abi = include_bytes!("erc20_abi.json").to_vec();
        info!("Loaded generic ABI");
        Ok(Web3Client {
            web3: Web3::new(transport),
            generic_abi: generic_abi.into(),
        })
    }

    pub async fn subscribe_logs<F>(
        &self,
        topics: Option<Vec<H256>>,
        process_log: Option<F>,
    ) -> web3::Result<()>
    where
        F: Fn(Log) + Send + 'static,
    {
        let topics = topics.unwrap_or_else(|| vec![
            // Default topics if none provided
            H256::from_str(TOPIC_V2_PAIR_CREATED).unwrap(),
            H256::from_str(TOPIC_V3_NEW_TOKEN).unwrap(),
            H256::from_str(TOPIC_OWNER_TRANSFER).unwrap(),
        ]);

        let filter = FilterBuilder::default()
            .topics(Some(topics), None, None, None)
            .build();

        
        let filter_req = self.web3.eth_filter().create_logs_filter(filter).await?;
        let filter_req_clone = filter_req.clone();
        let logs_stream = filter_req.stream(time::Duration::from_secs(60));
        futures::pin_mut!(logs_stream);

        loop {
            let ctrl_c = signal::ctrl_c();
            tokio::select! {
                Some(result_log) = logs_stream.next() => {
                    match result_log {
                        Ok(log) => {
                            if let Some(process) = &process_log {
                                process(log)
                            } // Optionally process the log
                        },
                        Err(e) => continue,
                    }
                }
                _ = ctrl_c => {
                    break;
                }
            }
        }

        // Uninstall the filter
        let _ = filter_req_clone.uninstall().await;

        Ok(())
    }

    pub async fn query_contract(&self, contract_address: Address, abi_path: Option<&str>) -> Result<(String, String, u8, U256), web3::Error> {
        
        info!("Querying contract for address: {:?}", contract_address);

        // Use specific ABI if provided, else use the generic ABI
        let abi = if let Some(path) = abi_path {
            info!("Using ABI from file: {}", path);
            std::fs::read(path)?.into()
        } else {
            info!("Using generic ABI");
            self.generic_abi.clone()
        };

        if !self.is_contract_deployed(contract_address).await? {
            return Err(Web3Error::InvalidResponse("No contract found at this address".into()));
        }

        info!("Contract is deployed at this address");

        let contract = Contract::from_json(self.web3.eth(), contract_address, &abi.0).unwrap();

        // Query the contract details
        let name: Result<String, _> = contract.query("name", (), None, Options::default(), None).await;
        if name.is_err() {
            error!("Error: {}", name.err().unwrap());
            return Err(Web3Error::InvalidResponse("Invalid function name".into()));
        }
        let symbol: Result<String, _> = contract.query("symbol", (), None, Options::default(), None).await;
        if symbol.is_err() {
            error!("Error: {}", symbol.err().unwrap());
            return Err(Web3Error::InvalidResponse("Invalid function symbol".into()));
        }
        
        let mut total_supply: Result<U256, _> = contract.query("totalSupply", (), None, Options::default(), None).await;
        if !total_supply.is_ok() {
            error!("Error: {}", total_supply.err().unwrap());
            //return Err(Web3Error::InvalidResponse("Invalid function totalSupply".into()));
            total_supply = Ok(U256::from(0));
        }
        let mut decimals: Result<u8, _> = contract.query("decimals", (), None, Options::default(), None).await;
        if !decimals.is_ok() {
            error!("Error: {}", decimals.err().unwrap());
            decimals = Ok(18);
        }

        let result = (name.unwrap(), symbol.unwrap(), decimals.unwrap(), total_supply.unwrap());
        info!("query contract for address: {:?} result: {:?}", contract_address, result);

        Ok(result)
    }
    
    async fn is_contract_deployed(&self,  contract_address: Address) -> Result<bool, web3::Error> {
        info!("Checking if contract is deployed at address: {:?}", contract_address);
        match self.web3.eth().code(contract_address, None).await {
            Ok(code) => {
                if code.0.is_empty() {
                    info!("No contract found at this address");
                    Ok(false)
                } else 
                {
                    let code_str = format!("{:?}", code);
                    info!("Code length: {}", code_str.len());
                    if code_str != "0x"
                    {
                        info!("Contract is deployed at this address");
                        Ok(true)
                    }
                    else {
                        info!("Contract is not deployed at this address");
                        Ok(false)
                    }
                }
                
            },
            Err(e) => {
                info!("Error occurred while checking if contract is deployed at address: {:?}", contract_address);
                Err(e)
            },
        }
    }

}
