use crate::{prelude::*, Result};

use drand_verify::g1_from_fixed;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct Randomness {
    pub round: u64,
    pub randomness: String,
    pub signature: String,
    pub previous_signature: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedRandomness {
    pub round: u64,
    pub public_key: [u8; 48],
    pub signature: [u8; 96],
    pub randomness: [u8; 32],
    pub previous_signature: [u8; 96],
}

impl Randomness {
    /// Verify randomness and return verified random bytes. If failed to verify
    /// then return None
    pub(crate) fn verify(&self, info: &ChainInfo) -> Result<Option<VerifiedRandomness>> {
        let round = self.round;
        let mut public_key = [0; 48];
        let mut signature = [0; 96];
        let mut randomness = [0; 32];
        let mut previous_signature = [0; 96];

        hex::decode_to_slice(&info.public_key, &mut public_key)?;
        hex::decode_to_slice(self.signature.as_str(), &mut signature)?;
        hex::decode_to_slice(self.randomness.as_str(), &mut randomness)?;
        hex::decode_to_slice(self.previous_signature.as_str(), &mut previous_signature)?;

        let pk =
            g1_from_fixed(public_key).map_err(|err| DrandError::InvalidPoint(err.to_string()))?;

        // verification is true
        if drand_verify::verify(&pk, round, &previous_signature, &signature)? {
            return Ok(Some(VerifiedRandomness {
                round,
                public_key,
                signature,
                randomness,
                previous_signature,
            }));
        }

        Ok(None)
    }
}
