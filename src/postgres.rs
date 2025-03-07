use crate::helpers::*;

#[test]
fn test_connect() {
    assert_timeout!(
        postgres::Config::new()
            .host(&connect_host())
            .connect_timeout(Duration::from_secs(1))
            .connect(postgres::NoTls),
        postgres::Error,
        "connection timed out"
    );
}
