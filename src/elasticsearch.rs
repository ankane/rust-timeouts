use crate::helpers::*;
use url::Url;

#[tokio::test]
async fn test_connect() {
    let url = Url::parse(&connect_url()).unwrap();
    let conn_pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let client = elasticsearch::Elasticsearch::new(transport);
    assert_timeout!(
        client.search(elasticsearch::SearchParts::None).send().await,
        "error sending request for url"
    );
}

#[tokio::test]
async fn test_read() {
    setup_read();

    let url = Url::parse(&read_url()).unwrap();
    let conn_pool = elasticsearch::http::transport::SingleNodeConnectionPool::new(url);
    let transport = elasticsearch::http::transport::TransportBuilder::new(conn_pool)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let client = elasticsearch::Elasticsearch::new(transport);
    assert_timeout!(
        client.search(elasticsearch::SearchParts::None).send().await,
        "error sending request for url"
    );
}
