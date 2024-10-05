use crate::helpers::*;

#[test]
fn test_connect() {
    let mut easy = curl::easy::Easy::new();
    easy.url(&connect_url()).unwrap();
    easy.connect_timeout(Duration::from_secs(1)).unwrap();
    assert_timeout!(easy.perform(), "Timeout was reached");
}

#[test]
fn test_read() {
    setup_read();

    let mut easy = curl::easy::Easy::new();
    easy.url(&read_url()).unwrap();
    easy.timeout(Duration::from_secs(1)).unwrap();
    assert_timeout!(easy.perform(), "Operation timed out");
}
