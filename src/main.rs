use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    
    let websocket = web3::transports::WebSocket::new(&env::var("KOVAN_ENDPOINT").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let address = Address::from_str("0x9326BFA02ADD2366b30bacB125260Af641031331").unwrap();
    let contract = Contract::from_json(web3s.eth(), address, include_bytes!("abi.json")).unwrap();

    let latest_data: (U256, U256, U256, U256, U256) = contract
        .query("latestRoundData", (), None, Options::default(), None)
        .await
        .unwrap();

    let fx_conv: U256 = U256::exp10(8);

    println!("ETH / USD latest trusted answer: ${}", latest_data.1.checked_div(fx_conv).unwrap());
    
    Ok(())
}