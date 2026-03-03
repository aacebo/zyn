use zyn::{to_camel_case, to_pascal_case, to_screaming_case, to_snake_case};

mod snake {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(to_snake_case("foo"), "foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(to_snake_case("foo_bar"), "foo_bar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(to_snake_case("fooBar"), "foo_bar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
    }

    #[test]
    fn from_screaming() {
        assert_eq!(to_snake_case("FOO_BAR"), "foo_bar");
    }

    #[test]
    fn no_dup_underscores() {
        assert_eq!(to_snake_case("foo__bar"), "foo_bar");
    }
}

mod pascal {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(to_pascal_case("foo"), "Foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(to_pascal_case("foo_bar"), "FooBar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(to_pascal_case("fooBar"), "FooBar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(to_pascal_case("FooBar"), "FooBar");
    }
}

mod camel {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(to_camel_case("foo"), "foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(to_camel_case("foo_bar"), "fooBar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(to_camel_case("FooBar"), "fooBar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(to_camel_case("fooBar"), "fooBar");
    }
}

mod screaming {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(to_screaming_case("foo"), "FOO");
    }

    #[test]
    fn from_snake() {
        assert_eq!(to_screaming_case("foo_bar"), "FOO_BAR");
    }

    #[test]
    fn from_camel() {
        assert_eq!(to_screaming_case("fooBar"), "FOO_BAR");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(to_screaming_case("FooBar"), "FOO_BAR");
    }
}
