use crate::helpers::*;

#[tokio::test]
async fn test_connect() {
    let client = reqwest::Client::new();
    assert_timeout!(
        client
            .get(connect_url())
            .timeout(Duration::from_secs(1))
            .send()
            .await,
        "operation timed out"
    );
}

#[tokio::test]
async fn test_connect_client() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    assert_timeout!(
        client.get(connect_url()).send().await,
        "operation timed out"
    );
}

#[tokio::test]
async fn test_read() {
    setup_read();

    let client = reqwest::Client::new();
    assert_timeout!(
        client
            .get(read_url())
            .timeout(Duration::from_secs(1))
            .send()
            .await,
        "operation timed out"
    );
}

#[tokio::test]
async fn test_read_client() {
    setup_read();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    assert_timeout!(client.get(read_url()).send().await, "operation timed out");
}
