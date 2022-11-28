use drand::Drand;

#[tokio::test]
async fn test_available_chains_latest() {
    let mut drand = Drand::available_chains()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain.latest().await.expect("Failed to retrieve info");

    latest.expect("Cannot get verified randomness");
}

#[tokio::test]
async fn test_league_of_entropy_drand() {
    let mut drand = Drand::loe_drand_chain()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain.latest().await.expect("Failed to retrieve info");

    latest.expect("Cannot get verified randomness");
}

#[tokio::test]
async fn test_specific_round() {
    let mut drand = Drand::loe_drand_chain()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain
        .round(3)
        .await
        .expect("Failed to retrieve info")
        .unwrap();

    // round is set by serialising, make sure we are at 3
    assert_eq!(latest.round, 3);
}
