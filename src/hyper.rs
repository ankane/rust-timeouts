use crate::helpers::*;

#[tokio::test]
async fn test_connect() {
    let client = hyper::Client::new();
    let uri = connect_url().parse().unwrap();
    assert_timeout!(
        tokio::time::timeout(Duration::from_secs(1), client.get(uri)).await,
        "deadline has elapsed"
    );
}

#[tokio::test]
async fn test_read() {
    setup_read();

    let client = hyper::Client::new();
    let uri = read_url().parse().unwrap();
    assert_timeout!(
        tokio::time::timeout(Duration::from_secs(1), client.get(uri)).await,
        "deadline has elapsed"
    );
}
