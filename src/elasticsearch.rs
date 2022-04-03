use crate::helpers;
use std::time::Duration;
use url::Url;

#[tokio::test]
async fn test_connect() {
    let url = Url::parse(&helpers::connect_url()).unwrap();
    let conn_pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let client = elasticsearch::Elasticsearch::new(transport);
    assert_timeout!(
        client.search(elasticsearch::SearchParts::None).send().await,
        "operation timed out"
    );
}

#[tokio::test]
async fn test_read() {
    helpers::setup_read();

    let url = Url::parse(&helpers::read_url()).unwrap();
    let conn_pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let client = elasticsearch::Elasticsearch::new(transport);
    assert_timeout!(
        client.search(elasticsearch::SearchParts::None).send().await,
        "operation timed out"
    );
}
