use crate::DrandError;
use crate::ENDPOINT;

use drand_verify::g1_from_fixed;
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

/// Unverified Randomness
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Randomness {
    pub round: u64,
    pub randomness: String,
    pub signature: String,
    pub previous_signature: String,
}

/// This is verified Randomness. To get a verified randomness, call verify by
/// passing the correct ChainInfo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedRandomness {
    pub public_key: [u8; 48],
    pub signature: [u8; 96],
    pub randomness: [u8; 32],
    pub previous_signature: [u8; 96],
}

impl Chain {
    /// Get latest round of randomness
    /// https://drand.love/developer/http-api/#chain-hash-public-latest
    pub async fn latest(&self) -> Result<Randomness, DrandError> {
        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", "latest");
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        Ok(randomness)
    }

    /// Get a specific round of randomness from the network. Round 0 gets the
    /// latest round
    pub async fn round(&self, round: usize) -> Result<Randomness, DrandError> {
        if round == 0 {
            return self.latest().await;
        }

        let url = format!("{}/{}/{}/{}", ENDPOINT, self.hash, "public", round);
        let value = reqwest::get(url).await?;

        let randomness: Randomness = value.json().await?;

        Ok(randomness)
    }
}

impl Randomness {
    /// Verify randomness and return verified random bytes. If failed to verify
    /// then return None
    pub fn verify(&self, info: ChainInfo) -> Result<Option<VerifiedRandomness>, DrandError> {
        let mut public_key = [0; 48];
        let mut signature = [0; 96];
        let mut randomness = [0; 32];
        let mut previous_signature = [0; 96];

        hex::decode_to_slice(info.public_key, &mut public_key)?;
        hex::decode_to_slice(self.signature.as_str(), &mut signature)?;
        hex::decode_to_slice(self.randomness.as_str(), &mut randomness)?;
        hex::decode_to_slice(self.previous_signature.as_str(), &mut previous_signature)?;

        let pk =
            g1_from_fixed(public_key).map_err(|err| DrandError::InvalidPoint(err.to_string()))?;

        // verification is true
        if drand_verify::verify(&pk, self.round, &previous_signature, &signature)? {
            return Ok(Some(VerifiedRandomness {
                public_key,
                signature,
                randomness,
                previous_signature,
            }));
        }

        Ok(None)
    }
}

/// Get request to get info for the hash.
pub(crate) async fn retrieve_info(hash: &str) -> Result<ChainInfo, DrandError> {
    let url = format!("{}/{}/{}", ENDPOINT, hash, "info");
    let value = reqwest::get(url).await?;
    let info: ChainInfo = value.json().await?;

    Ok(info)
}
