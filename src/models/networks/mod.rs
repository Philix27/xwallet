use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoNetwork {
    pub id: i64,
    pub name: String,
    pub chain_id: String,
    pub rpc_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkList {
    Polygon,
    Mumbai,
    Akash,
    Bnb,
}
