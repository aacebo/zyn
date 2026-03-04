#[test]
fn basic() {
    let name = zyn::quote::format_ident!("User");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "Users");
}

#[test]
fn already_ends_with_s() {
    let name = zyn::quote::format_ident!("Status");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "Statuses");
}

#[test]
fn lowercase() {
    let name = zyn::quote::format_ident!("item");
    let result = zyn::zyn!({ { name | plural } });
    assert_eq!(result.to_string(), "items");
}
