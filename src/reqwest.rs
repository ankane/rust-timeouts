use crate::helpers;
use std::time::Duration;

#[test]
fn test_connect() {
    let client = reqwest::blocking::Client::new();
    assert_timeout!(
        client
            .get(helpers::connect_url())
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
        client.get(helpers::connect_url()).send(),
        "operation timed out"
    );
}

#[test]
fn test_read() {
    helpers::setup_read();

    let client = reqwest::blocking::Client::new();
    assert_timeout!(
        client
            .get(helpers::read_url())
            .timeout(Duration::from_secs(1))
            .send(),
        "operation timed out"
    );
}

#[test]
fn test_read_client() {
    helpers::setup_read();

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    assert_timeout!(
        client.get(helpers::read_url()).send(),
        "operation timed out"
    );
}
