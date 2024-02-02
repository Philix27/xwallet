use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoNetwork {
    name: String,
    chain_id: String,
    rpc_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkList {
    Polygon,
    Mumbai,
    Akash,
    Bnb,
}
