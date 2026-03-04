#[test]
fn strips_trailing_s() {
    let name = zyn::quote::format_ident!("users");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "user");
}

#[test]
fn preserves_double_s() {
    let name = zyn::quote::format_ident!("class");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "class");
}

#[test]
fn no_trailing_s() {
    let name = zyn::quote::format_ident!("item");
    let result = zyn::zyn!({ { name | singular } });
    assert_eq!(result.to_string(), "item");
}
