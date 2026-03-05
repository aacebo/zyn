use zyn_core::ast::Element;

#[test]
fn note_and_help_produce_distinct_output() {
    let note_ts = zyn::parse!("@note \"msg\"" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    let help_ts = zyn::parse!("@help \"msg\"" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert_ne!(
        note_ts, help_ts,
        "@note and @help must produce distinct tokens"
    );
}

#[test]
fn throw_and_warn_produce_distinct_output() {
    let throw_ts = zyn::parse!("@throw \"msg\"" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    let warn_ts = zyn::parse!("@warn \"msg\"" => Element)
        .unwrap()
        .to_token_stream()
        .to_string();
    assert_ne!(
        throw_ts, warn_ts,
        "@throw and @warn must produce distinct tokens"
    );
}
