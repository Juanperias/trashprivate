use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Crypto {
    #[serde(rename = "current_price")]
    pub price: f32,
    pub market_cap: f32,
    #[serde(rename = "circulating_supply")]
    pub supply: f32,
    pub symbol: String,
}
