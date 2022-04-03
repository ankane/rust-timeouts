use crate::helpers;
use std::time::Duration;

#[test]
fn test_connect() {
    assert_timeout!(
        postgres::Config::new()
            .host(&helpers::connect_host())
            .connect_timeout(Duration::from_secs(1))
            .connect(postgres::NoTls),
        "connection timed out"
    );
}
