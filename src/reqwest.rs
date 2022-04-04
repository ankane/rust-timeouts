use crate::helpers::*;

#[test]
fn test_connect() {
    let client = reqwest::blocking::Client::new();
    assert_timeout!(
        client
            .get(connect_url())
            .timeout(Duration::from_secs(1))
            .send(),
        "operation timed out"
    );
}

#[test]
fn test_connect_client() {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    assert_timeout!(
        client.get(connect_url()).send(),
        "operation timed out"
    );
}

#[test]
fn test_read() {
    setup_read();

    let client = reqwest::blocking::Client::new();
    assert_timeout!(
        client
            .get(read_url())
            .timeout(Duration::from_secs(1))
            .send(),
        "operation timed out"
    );
}

#[test]
fn test_read_client() {
    setup_read();

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    assert_timeout!(
        client.get(read_url()).send(),
        "operation timed out"
    );
}
