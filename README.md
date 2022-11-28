# Drand Client

This is a drand client made for rust. It uses reqwest for sending HTTP requests and gets randomness data from the Drand API. It allows performing verification using
the [drand-verify](https://github.com/noislabs/drand-verify) crate. 

To access an available chain from the API (https://drand.cloudflare.com/chains)
```rust
use drand::Drand;

#[tokio::main]
async fn main() {
    let mut drand = Drand::available_chains()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0); // drand.chains is a Vec<Chains> to interact with chains 

    let latest = chain.latest().await.expect("Failed to retrieve info").unwrap(); // get the latest round of verified randmoness from the chain
}

```

`chain.latest()` will give you `Result<OptionVerifiedRandomness>>` which is verified autoamtically, unverified randomnesses are not supported. If there's an error in verification then the error the propogated using the `drand::DrandError` enum.

## LICENSE

MIT
