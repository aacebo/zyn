use zyn::FromArg;
use zyn::meta::Args;

#[derive(zyn::Attribute)]
#[zyn("my_attr", about = "test attribute")]
struct MyAttr {
    #[zyn(about = "the name")]
    name: String,
    #[zyn(default)]
    count: i64,
    enabled: bool,
    tag: Option<String>,
}

#[test]
fn attribute_mode_full_extraction() {
    let args: Args = zyn::parse!("name = \"hello\", count = 3, enabled, tag = \"v1\"").unwrap();
    let attr = MyAttr::from_args(&args).unwrap();

    assert_eq!(attr.name, "hello");
    assert_eq!(attr.count, 3);
    assert!(attr.enabled);
    assert_eq!(attr.tag, Some("v1".to_string()));
}

#[test]
fn attribute_mode_optional_absent() {
    let args: Args = zyn::parse!("name = \"hi\"").unwrap();
    let attr = MyAttr::from_args(&args).unwrap();

    assert_eq!(attr.name, "hi");
    assert_eq!(attr.count, 0);
    assert!(!attr.enabled);
    assert_eq!(attr.tag, None);
}

#[test]
fn attribute_mode_missing_required_is_err() {
    let args: Args = zyn::parse!("").unwrap();
    assert!(MyAttr::from_args(&args).is_err());
}

#[derive(zyn::Attribute)]
struct ArgMode {
    a: i64,
    b: String,
}

#[test]
fn argument_mode_from_args() {
    let args: Args = zyn::parse!("a = 42, b = \"hello\"").unwrap();
    let v = ArgMode::from_args(&args).unwrap();

    assert_eq!(v.a, 42);
    assert_eq!(v.b, "hello");
}

#[test]
fn argument_mode_from_arg_via_list() {
    let arg: zyn::meta::Arg = zyn::parse!("inner(a = 7, b = \"world\")").unwrap();
    let v = ArgMode::from_arg(&arg).unwrap();

    assert_eq!(v.a, 7);
    assert_eq!(v.b, "world");
}

#[test]
fn argument_mode_from_arg_non_list_is_err() {
    let arg: zyn::meta::Arg = zyn::parse!("skip").unwrap();
    assert!(ArgMode::from_arg(&arg).is_err());
}

#[derive(zyn::Attribute)]
#[zyn("outer")]
struct Outer {
    inner: ArgMode,
}

#[test]
fn recursive_nesting() {
    let args: Args = zyn::parse!("inner(a = 5, b = \"nested\")").unwrap();
    let v = Outer::from_args(&args).unwrap();

    assert_eq!(v.inner.a, 5);
    assert_eq!(v.inner.b, "nested");
}

#[derive(zyn::Attribute)]
#[zyn("positioned")]
struct Positional {
    #[zyn(0)]
    first: String,
    #[zyn(1)]
    second: i64,
}

#[test]
fn positional_args() {
    let args: Args = zyn::parse!("\"hello\", 42").unwrap();
    let v = Positional::from_args(&args).unwrap();

    assert_eq!(v.first, "hello");
    assert_eq!(v.second, 42);
}

#[derive(zyn::Attribute)]
#[zyn("renamed")]
struct Renamed {
    #[zyn("my_key")]
    value: String,
}

#[test]
fn name_override() {
    let args: Args = zyn::parse!("my_key = \"found\"").unwrap();
    let v = Renamed::from_args(&args).unwrap();

    assert_eq!(v.value, "found");
}

#[derive(zyn::Attribute)]
#[zyn("defaulted")]
struct WithDefault {
    #[zyn(default = "fallback")]
    label: String,
    #[zyn(default)]
    count: i64,
}

#[test]
fn default_expr_used_when_absent() {
    let args: Args = zyn::parse!("").unwrap();
    let v = WithDefault::from_args(&args).unwrap();

    assert_eq!(v.label, "fallback");
    assert_eq!(v.count, 0);
}

#[test]
fn default_overridden_when_present() {
    let args: Args = zyn::parse!("label = \"custom\", count = 9").unwrap();
    let v = WithDefault::from_args(&args).unwrap();

    assert_eq!(v.label, "custom");
    assert_eq!(v.count, 9);
}

#[derive(zyn::Attribute)]
#[zyn("skipped")]
struct WithSkip {
    name: String,
    #[zyn(skip)]
    internal: i64,
}

#[test]
fn skip_field_always_default() {
    let args: Args = zyn::parse!("name = \"hi\"").unwrap();
    let v = WithSkip::from_args(&args).unwrap();

    assert_eq!(v.name, "hi");
    assert_eq!(v.internal, 0);
}

#[test]
fn about_generated() {
    let about = MyAttr::about();

    assert!(about.contains("#[my_attr(...)]"));
    assert!(about.contains("test attribute"));
    assert!(about.contains("name"));
    assert!(about.contains("the name"));
}

#[test]
fn about_shows_required_status() {
    let about = MyAttr::about();
    assert!(about.contains("(required)"));
}

fn expect_err<T>(result: zyn::Result<T>) -> zyn::Diagnostic {
    match result {
        Err(e) => e,
        Ok(_) => panic!("expected error"),
    }
}

#[test]
fn multiple_missing_fields_all_reported() {
    let args: Args = zyn::parse!("").unwrap();
    let err = expect_err(ArgMode::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("missing required field `a`"));
    assert!(combined.contains("missing required field `b`"));
    assert_eq!(err.len(), 2);
}

#[test]
fn unknown_key_reported() {
    let args: Args = zyn::parse!("name = \"hello\", bogus = 1").unwrap();
    let err = expect_err(MyAttr::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("unknown argument `bogus`"));
}

#[test]
fn did_you_mean_suggestion() {
    let args: Args = zyn::parse!("naem = \"hello\"").unwrap();
    let err = expect_err(MyAttr::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("unknown argument `naem`"));
    assert!(combined.contains("did you mean"));
}

#[test]
fn unknown_key_no_suggestion_when_distant() {
    let args: Args = zyn::parse!("name = \"hello\", zzzzzzzzz = 1").unwrap();
    let err = expect_err(MyAttr::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("unknown argument `zzzzzzzzz`"));
}

#[test]
fn about_text_in_missing_error() {
    let args: Args = zyn::parse!("count = 1").unwrap();
    let err = expect_err(MyAttr::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("missing required field `name`"));
    assert!(combined.contains("the name"));
}

#[test]
fn valid_args_still_work_after_accumulation() {
    let args: Args = zyn::parse!("name = \"test\", count = 5, enabled, tag = \"v2\"").unwrap();
    let attr = MyAttr::from_args(&args).unwrap();

    assert_eq!(attr.name, "test");
    assert_eq!(attr.count, 5);
    assert!(attr.enabled);
    assert_eq!(attr.tag, Some("v2".to_string()));
}

#[test]
fn type_mismatch_and_missing_both_reported() {
    let args: Args = zyn::parse!("a = \"not_an_int\", b = 42").unwrap();
    let err = expect_err(ArgMode::from_args(&args));
    let combined = format!("{err}");

    assert!(err.is_error());
    assert!(combined.contains("expected integer literal"));
    assert!(combined.contains("expected string literal"));
    assert_eq!(err.len(), 2);
}

#[test]
fn unknown_key_with_valid_fields_still_errors() {
    let args: Args = zyn::parse!("a = 1, b = \"ok\", bogus = 3").unwrap();
    let err = expect_err(ArgMode::from_args(&args));
    let combined = format!("{err}");

    assert!(combined.contains("unknown argument `bogus`"));
    assert_eq!(err.len(), 1);
}
