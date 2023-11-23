use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use web3::ethabi::token;
use dotenv::dotenv;
use env_logger;

#[derive(Serialize, Deserialize, Debug)]
pub struct EtherscanTokenTx {
    status: String,
    message: String,
    result: Vec<EtherscanTransactionResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EtherscanAbi {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtherscanTransactionResult {
    block_number: String,
    time_stamp: String,
    hash: String,
    nonce: String,
    block_hash: String,
    from: String,
    contract_address: String,
    to: String,
    value: String,
    token_name: String,
    token_symbol: String,
    token_decimal: String,
    transaction_index: String,
    gas: String,
    gas_price: String,
    gas_used: String,
    cumulative_gas_used: String,
    input: String,
    confirmations: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtherscanTokenInfo {
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimal: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EtherscanQuery {
    module: Option<String>,
    action: Option<String>,
    contract_address: Option<String>,
    address: Option<String>,
    page: Option<u8>,
    offset: Option<u8>,
    sort: Option<String>,
    api_key: Option<String>,
    start_block: Option<String>,
    end_block: Option<String>
}

pub struct EtherscanClient {
    api_key: String,
}
impl EtherscanClient {
    pub fn new(api_key: String) -> Self {
        EtherscanClient { api_key }
    }
    pub fn resolve_url(&self, query: EtherscanQuery) -> String {
        //Check each property empty or not and add to url, as a vec and join by &
        let mut url = String::from("https://api.etherscan.io/api?");
        let mut query_vec = Vec::new();
        
        if !query.module.is_none() {
            query_vec.push(format!("module={}", query.module.unwrap()));
        }
        if !query.action.is_none() {
            query_vec.push(format!("action={}", query.action.unwrap()));
        }
        if !query.contract_address.is_none() {
            query_vec.push(format!("contractaddress={}", query.contract_address.unwrap()));
        }
        if !query.address.is_none() {
            query_vec.push(format!("address={}", query.address.unwrap()));
        }
        
        if !query.start_block.is_none() {
            query_vec.push(format!("startblock={}", query.start_block.unwrap()));
        }

        if !query.end_block.is_none() {
            query_vec.push(format!("endblock={}", query.end_block.unwrap()));
        }

        if !query.page.is_none() {
            let val = query.page.unwrap();
            if val > 0 {
                query_vec.push(format!("page={}", val));
            }
        }
        if !query.offset.is_none() {
            let val = query.offset.unwrap();
            if val > 0 {
                query_vec.push(format!("offset={}", val));
            }
        }
        
        if !query.sort.is_none() {
            query_vec.push(format!("sort={}", query.sort.unwrap()));
        }

        if !query.api_key.is_none() {
            query_vec.push(format!("apikey={}", query.api_key.unwrap()));
        }
        else {
            if !self.api_key.is_empty() {
                query_vec.push(format!("apikey={}", self.api_key));    
            }
        }
        
        
        url.push_str(&query_vec.join("&"));
        url
    }

    pub async fn get_token_verified(&self, address: String) -> Result<EtherscanAbi, Box<dyn std::error::Error>> {
        let mut query = EtherscanQuery :: default();
        query.address = Some(address);
        if query.address.is_none() {
            return Err(format!("Contract address is empty").into());
        }
        query.module = Some(String::from("contract"));
        query.action = Some(String::from("getabi"));
        let url = self.resolve_url(query);
        info!("URL: {:?}", url);
        let response = reqwest::get(&url).await?.json::<EtherscanAbi>().await?;
        Ok(response)
        
    }

    pub async fn get_token_transactions(&self, 
        mut query: EtherscanQuery,
    ) -> Result<EtherscanTokenTx, reqwest::Error> {
        query.module = Some(String::from("account"));
        query.action = Some(String::from("tokentx"));
        let url = self.resolve_url(query);
        info!("URL: {:?}", url);
        let response = reqwest::get(&url).await?.json::<EtherscanTokenTx>().await?;
        Ok(response)
    }

    pub async fn get_token_info(&self, 
        contract_address: &str,
    ) -> Result<EtherscanTokenInfo, Box<dyn std::error::Error>> {
        info!("Getting token info for address: {:?}", contract_address);
        let mut query = EtherscanQuery :: default();

        query.module = Some(String::from("token"));
        query.action = Some(String::from("tokeninfo"));
        query.contract_address = Some(String::from(contract_address));
            
        let response = self.get_token_transactions(query).await?;
        if response.result.is_empty() {
            return Err(format!("No token info found for address: {:?}", contract_address).into());
        }
        let token_info = EtherscanTokenInfo {
            contract_address: response.result[0].contract_address.clone(),
            token_name: response.result[0].token_name.clone(),
            token_symbol: response.result[0].token_symbol.clone(),
            token_decimal: response.result[0].token_decimal.clone(),
        };
        
        Ok(token_info)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use log::info;
    use serde_json::Value;

    // Common setup function
    fn setup() -> EtherscanClient {
        dotenv().ok();
        env_logger::init();

        let api_key = std::env::var("ETHERSCAN_API_KEY").unwrap_or_else(|_| String::from("default_api_key"));


        info!("Setting up Etherscan client... with api key {:?}", api_key);
        EtherscanClient::new(String::from(api_key))
    }

    // #[test]
    // fn test_resolve_url_valid() {
    //     let client = setup();
    //     let query = EtherscanQuery::default();
    //     let url = client.resolve_url(&query);
    //     info!("Resolved URL: {}", url);
    //     // Assertions...
    // }

    #[tokio::test]
    async fn test_get_token_verified_valid() {
        let client = setup();
        info!("*************** Testing get_token_verified_valid ***************");
        
        let response = client.get_token_verified(String::from("0x31d3bb633b67549305d64ac348e296541eacc832")).await.unwrap();
       // info!("Response: {:?}", response);
        // Assertions...

        assert_eq!(response.status, "1");

        let abi:  Result<Value, serde_json::Error> = serde_json::from_str(&response.result);
                                match abi {
                                    Ok(parsed_json) => {
                                        info!("ABI: {:?}", parsed_json);
                                        
                                        assert!(true, "ABI parsed successfully ")
                                    },
                                    Err(e) => assert!(false, "Error parsing ABI: {:?}", e)
                                }
    }

    #[tokio::test]
    async fn test_get_token_verified_invalid() {
        let client = setup();
        let response = client.get_token_verified(String::from("0xF2a6D940b2228586FdDd3795634E6a24e9B3A0E4")).await.unwrap();
        info!("Response: {:?}", response);
        // Assertions...
        assert_eq!(response.status, "0");
    }
    // Similar structure for other tests
}
