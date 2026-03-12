use builder::Builder;

#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    #[builder(default)]
    verbose: bool,
    #[builder(default_value = "30")]
    timeout: i64,
}

#[test]
fn builds_with_all_fields() {
    let config = Config::builder()
        .host("localhost".to_string())
        .port(8080)
        .verbose(true)
        .timeout(60)
        .build();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert!(config.verbose);
    assert_eq!(config.timeout, 60);
}

#[test]
fn uses_defaults_when_omitted() {
    let config = Config::builder()
        .host("localhost".to_string())
        .port(8080)
        .build();

    assert!(!config.verbose);
    assert_eq!(config.timeout, 30);
}

#[test]
#[should_panic(expected = "field `host` is required")]
fn panics_on_missing_required_field() {
    Config::builder().port(8080).build();
}
