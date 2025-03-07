use crate::helpers::*;

#[test]
fn test_connect() {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(1)))
        .build();
    let agent: ureq::Agent = config.into();
    assert_timeout!(agent.get(&connect_url()).call(), "timeout: global");
}

#[test]
fn test_read() {
    setup_read();

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(1)))
        .build();
    let agent: ureq::Agent = config.into();
    assert_timeout!(agent.get(&read_url()).call(), "timeout: global");
}
