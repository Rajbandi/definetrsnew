mod etherscan;
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

  
    
    let response = etherscan::get_token_info(contract_address).await?;
    
    println!("{:#?}", response);
    
    Ok(())
}
