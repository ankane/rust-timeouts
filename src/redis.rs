use crate::helpers::*;

#[test]
fn test_connect() {
    let client = redis::Client::open(format!("redis://{}/", connect_host_and_port())).unwrap();
    assert_timeout!(
        client.get_connection_with_timeout(Duration::from_secs(1)),
        "connection timed out"
    );
}

#[test]
fn test_read() {
    setup_read();

    let client =
        redis::Client::open(format!("redis://{}/", read_host_and_port())).unwrap();
    let mut con = client.get_connection().unwrap();
    con.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    con.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
    assert_timeout!(
        redis::cmd("GET").arg("key").query::<()>(&mut con),
        "Resource temporarily unavailable"
    );
}
