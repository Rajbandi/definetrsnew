
use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Serialize, Deserialize, Debug)]
pub struct EtherscanTokenTx {
    status: String,
    message: String,
    result: Vec<EtherscanTransactionResult>,
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
    contract_address: String,
    token_name: String,
    token_symbol: String,
    token_decimal: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtherscanQuery {
    module: String,
    action: String,
    contract_address: String,
    address: String,
    page: u8,
    offset: u8,
    sort: String,
    api_key: String,
    start_block: String,
    end_block: String,
}

pub fn resolve_url(query: EtherscanQuery) -> String {

    //Check each property empty or not and add to url, as a vec and join by &
    let mut url = String::from("https://api.etherscan.io/api?");
    let mut query_vec = Vec::new();
    if !query.module.is_empty() {
        query_vec.push(format!("module={}", query.module));
    }
    if !query.action.is_empty() {
        query_vec.push(format!("action={}", query.action));
    }
    if !query.contract_address.is_empty() {
        query_vec.push(format!("contractaddress={}", query.contract_address));
    }
    if !query.address.is_empty() {
        query_vec.push(format!("address={}", query.address));
    }
    if !query.start_block.is_empty() {
        query_vec.push(format!("startblock={}", query.start_block));
    }
    if !query.end_block.is_empty() {
        query_vec.push(format!("endblock={}", query.end_block));
    }

    if query.page != 0 {
        query_vec.push(format!("page={}", query.page));
    }
    if query.offset != 0 {
        query_vec.push(format!("offset={}", query.offset));
    }
    if !query.sort.is_empty() {
        query_vec.push(format!("sort={}", query.sort));
    }
    if !query.api_key.is_empty() {
        query_vec.push(format!("apikey={}", query.api_key));
    }
    url.push_str(&query_vec.join("&"));
    url
    

    
}

//Write a new method like get_token_transactions accepting page and offset, startBlock, endBlock as parameters

pub async fn get_token_transactions(mut query: EtherscanQuery) -> Result<EtherscanTokenTx, reqwest::Error> {
    query.module = String::from("account");
    query.action = String::from("tokentx");
    let url = resolve_url(query);
    let response = reqwest::get(&url).await?.json::<EtherscanTokenTx>().await?;
    Ok(response)
}

pub async fn get_token_info(contract_address: &str) -> Result<EtherscanTokenInfo, reqwest::Error> {
   let query = EtherscanQuery {
        module: String::from("token"),
        action: String::from("tokeninfo"),
        contract_address: String::from(contract_address),
        address: String::from(""),
        page: 0,
        offset: 0,
        sort: String::from(""),
        api_key: String::from(""),
        start_block: String::from(""),
        end_block: String::from(""),
    };
    let response = get_token_transactions(query).await?;
    let token_info = EtherscanTokenInfo {
        contract_address: response.result[0].contract_address.clone(),
        token_name: response.result[0].token_name.clone(),
        token_symbol: response.result[0].token_symbol.clone(),
        token_decimal: response.result[0].token_decimal.clone(),
    };
    Ok(token_info)
}