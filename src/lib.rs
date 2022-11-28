mod chain;
mod randomness;

pub mod prelude {
    pub use crate::chain::{Chain, ChainInfo};
    pub use crate::randomness::VerifiedRandomness;
    pub use crate::{Drand, DrandError};
}

use crate::chain::{retrieve_info, Chain};

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use drand_verify::VerificationError;
use hex::FromHexError;
use serde_json::Value;

pub type Result<T> = std::result::Result<T, DrandError>;

const ENDPOINT: &str = "https://drand.cloudflare.com";
const LEAGUE_OF_ENTROPY_HASH: &str =
    "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drand {
    pub chains: Vec<Chain>,
}

#[derive(Debug)]
pub enum DrandError {
    /// Error when sending a HTTP request
    RequestError(reqwest::Error),
    /// Invalid response from drand api
    InvalidResponse(String),
    /// Error in converting the hashes to hex from the API
    HexError(FromHexError),
    /// Error when randomness cannot be verified
    VerificationError(VerificationError),
    /// InvalidPoint error from g1_from_fixed function
    InvalidPoint(String),
}

impl Drand {
    /// Get available chains from `drand.cloudflare.com/chains`
    pub async fn available_chains() -> Result<Self> {
        let url = format!("{}/{}", ENDPOINT, "chains");
        let value = reqwest::get(url).await?;
        let json = value.json::<Value>().await?;

        if let Value::Array(arr) = json {
            let mut chains = Vec::new();

            // No combinators because it's easier to deal with in async code
            for chain in arr {
                if let Value::String(hash) = chain {
                    let info = retrieve_info(&hash).await?;

                    chains.push(Chain { hash, info })
                }
            }

            return Ok(Self { chains });
        }

        Err(DrandError::from(json.to_string()))
    }

    /// Get the League of Entropy drand group
    /// https://drand.love/developer/http-api/#public-endpoints
    pub async fn loe_drand_chain() -> Result<Self> {
        let hash = String::from(LEAGUE_OF_ENTROPY_HASH);
        let info = retrieve_info(&hash).await?;

        let chain = Chain { hash, info };

        Ok(Self {
            chains: vec![chain],
        })
    }
}

impl From<reqwest::Error> for DrandError {
    fn from(err: reqwest::Error) -> Self {
        DrandError::RequestError(err)
    }
}

impl From<String> for DrandError {
    fn from(err: String) -> Self {
        DrandError::InvalidResponse(err)
    }
}

impl From<VerificationError> for DrandError {
    fn from(err: VerificationError) -> Self {
        DrandError::VerificationError(err)
    }
}

impl From<FromHexError> for DrandError {
    fn from(err: FromHexError) -> Self {
        DrandError::HexError(err)
    }
}

impl Display for DrandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x = match self {
            Self::RequestError(err) => format!("Reqwest error: {}", err),
            Self::InvalidResponse(err) => format!("Invalid response: {}", err),
            Self::HexError(err) => format!("Cannot convert to hex: {}", err),
            Self::VerificationError(err) => format!("Cannot verify randomness: {}", err),
            Self::InvalidPoint(err) => format!("Invalid Point: {}", err),
        };

        write!(f, "{}", x)
    }
}

impl Error for DrandError {}
