mod etherscan;
use etherscan::EtherscanClient;
use serde::{Serialize, Deserialize};
use reqwest;
use tokio;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Usage: program <contract_address>");
    //     return Ok(());
    // }
    let contract_address = "0xFc99C1E310758CCD0BE1866aB267C1e54Ce4f460";

  
    let api_key = "D1ZAF93H82CVWHGZ37J7EKM3RKWAD3323W";
    let client = EtherscanClient::new(api_key.to_string());

    match client.get_token_info(contract_address).await {
        Ok(response) => println!("{:#?}", response),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
    
    Ok(())
}
