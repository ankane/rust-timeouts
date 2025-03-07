use crate::helpers::*;
use std::io::Read;

#[test]
fn test_connect() {
    let addr: std::net::SocketAddr = connect_host_and_port().parse().unwrap();
    assert_timeout!(
        std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(1)),
        std::io::Error,
        "connection timed out"
    );
}

#[test]
fn test_read() {
    setup_read();

    let mut stream = std::net::TcpStream::connect(&read_host_and_port()).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    let mut buf = Vec::new();
    assert_timeout!(
        stream.read_to_end(&mut buf),
        std::io::Error,
        "Resource temporarily unavailable"
    );
}
