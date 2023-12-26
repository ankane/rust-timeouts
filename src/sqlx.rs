use crate::helpers::*;

#[tokio::test]
async fn test_connect() {
    assert_timeout!(
        sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(1))
            .connect(&format!("postgres://{}/dbname", connect_host_and_port()))
            .await,
        "pool timed out while waiting for an open connection"
    );
}
