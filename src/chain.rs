use crate::randomness::Randomness;
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
    pub async fn latest(&self) -> Result<Option<VerifiedRandomness>> {
        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", "latest");
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        randomness.verify(&self.info)
    }

    /// Get a specific round of randomness from the network. Round 0 gets the
    /// latest round
    pub async fn round(&self, round: usize) -> Result<Option<VerifiedRandomness>> {
        if round == 0 {
            return self.latest().await;
        }

        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", round);
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        randomness.verify(&self.info)
    }
}
