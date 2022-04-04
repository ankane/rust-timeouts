use crate::helpers;
use std::io::Read;
use std::time::Duration;

#[test]
fn test_connect() {
    let addr: std::net::SocketAddr = helpers::connect_host_and_port().parse().unwrap();
    assert_timeout!(
        std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(1)),
        "connection timed out"
    );
}

#[test]
fn test_read() {
    helpers::setup_read();

    let mut stream = std::net::TcpStream::connect(&helpers::read_host_and_port()).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
    let mut buf = Vec::new();
    assert_timeout!(
        stream.read_to_end(&mut buf),
        "Resource temporarily unavailable"
    );
}
