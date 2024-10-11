use crate::models::crypto::Crypto;
use anyhow::Result;

const BASE_URL: &str = "https://api.coingecko.com/api/v3";

pub fn ping() -> Result<()> {
    let url = format!("{BASE_URL}/ping");
    ureq::get(&url).call()?;

    Ok(())
}

pub fn get_cryptos() -> Result<Vec<Crypto>> {
    let url = format!("{BASE_URL}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=false");
    let res = ureq::get(&url).call()?.into_string()?;
    let data: Vec<Crypto> = serde_json::from_str(&res)?;

    Ok(data)
}
