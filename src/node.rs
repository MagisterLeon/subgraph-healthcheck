use std::env;

use web3::types::{U64};

#[tokio::main]
pub(crate) async fn get_block_number() -> web3::Result<U64> {
    let mainnet_url = &env::var("MAINNET_URL").unwrap();
    let transport = web3::transports::Http::new(mainnet_url)?;
    let web3 = web3::Web3::new(transport);

    web3.eth().block_number().await
}