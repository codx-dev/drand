use crate::ENDPOINT;
use crate::{prelude::*, Result};

use serde::Deserialize;

/// Represents a chain, the info will be None if it has not been retrived yet.
/// Call the `retrieve_info` method to get info.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Chain {
    pub hash: String,
    pub info: ChainInfo,
}

/// Represents the info of a particular chain.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ChainInfo {
    pub public_key: String,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: String,
}

impl Chain {
    /// Get latest round of randomness
    /// https://drand.love/developer/http-api/#chain-hash-public-latest
    pub async fn latest(&self) -> Result<Randomness> {
        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", "latest");
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        Ok(randomness)
    }

    /// Get a specific round of randomness from the network. Round 0 gets the
    /// latest round
    pub async fn round(&self, round: usize) -> Result<Randomness> {
        if round == 0 {
            return self.latest().await;
        }

        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", round);
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        Ok(randomness)
    }
}

/// Get request to get info for the hash.
pub(crate) async fn retrieve_info(hash: &str) -> Result<ChainInfo> {
    let url = format!("{}/{}/{}", ENDPOINT, hash, "info");
    let value = reqwest::get(url).await?;
    let info: ChainInfo = value.json().await?;

    Ok(info)
}
