use crate::api::{get_cryptos, ping};
use anyhow::Result;
use log::error;
pub fn data() -> Result<()> {
    if !ping().is_ok() {
        error!("coin gecko api not responding")
    }

    let cryptos = get_cryptos()?;

    Ok(())
}
