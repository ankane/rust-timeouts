use crate::helpers;
use std::time::Duration;

#[test]
fn test_connect() {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(1))
        .build();
    assert_timeout!(
        agent.get(&helpers::connect_url()).call(),
        "connection timed out"
    );
}

#[test]
fn test_read() {
    helpers::setup_read();

    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(1))
        .timeout_write(Duration::from_secs(1))
        .build();
    assert_timeout!(
        agent.get(&helpers::read_url()).call(),
        "timed out reading response"
    );
}
