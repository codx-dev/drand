use drand::Drand;

#[tokio::test]
async fn test_available_chains_latest() {
    let mut drand = Drand::available_chains()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain.latest().await.expect("Failed to retrieve info");

    latest.verify(chain.info).expect("Cannot verify").unwrap();
}

#[tokio::test]
async fn test_league_of_entropy_drand() {
    let mut drand = Drand::loe_drand_chain()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain.latest().await.expect("Failed to retrieve info");

    latest.verify(chain.info).expect("Cannot verify").unwrap();
}

#[tokio::test]
async fn test_specific_round() {
    let mut drand = Drand::loe_drand_chain()
        .await
        .expect("Cannot find available chains");

    let chain = drand.chains.remove(0);

    let latest = chain.round(3).await.expect("Failed to retrieve info");

    // round is set by serialising, make sure we are
    assert_eq!(latest.round, 3);

    latest.verify(chain.info).expect("Cannot verify").unwrap();
}
