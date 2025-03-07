use crate::helpers::*;

#[actix_rt::test]
async fn test_connect() {
    let client = awc::Client::builder()
        .timeout(Duration::from_secs(1))
        .finish();
    assert_timeout!(
        client.get(&connect_url()).send().await,
        awc::error::SendRequestError,
        "Timeout while waiting for response"
    );
}

#[actix_rt::test]
async fn test_read() {
    setup_read();

    let client = awc::Client::builder()
        .timeout(Duration::from_secs(1))
        .finish();
    assert_timeout!(
        client.get(&read_url()).send().await,
        awc::error::SendRequestError,
        "Timeout while waiting for response"
    );
}
