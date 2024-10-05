use crate::helpers::*;

#[test]
fn test_connect() {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(1))
        .build();
    assert_timeout!(agent.get(&connect_url()).call(), "connection timed out");
}

#[test]
fn test_read() {
    setup_read();

    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(1))
        .timeout_write(Duration::from_secs(1))
        .build();
    assert_timeout!(agent.get(&read_url()).call(), "timed out reading response");
}
