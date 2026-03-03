use zyn_core::{Camel, Kebab, Pascal, Pipe, Screaming, Snake};

mod snake {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(Snake.pipe("foo".to_string()).to_string(), "foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(Snake.pipe("foo_bar".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(Snake.pipe("fooBar".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(Snake.pipe("FooBar".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn from_screaming() {
        assert_eq!(Snake.pipe("FOO_BAR".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn no_dup_underscores() {
        assert_eq!(Snake.pipe("foo__bar".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn consecutive_uppercase() {
        assert_eq!(
            Snake.pipe("HTTPResponse".to_string()).to_string(),
            "http_response"
        );
    }

    #[test]
    fn consecutive_uppercase_xml() {
        assert_eq!(
            Snake.pipe("XMLParser".to_string()).to_string(),
            "xml_parser"
        );
    }

    #[test]
    fn all_uppercase() {
        assert_eq!(Snake.pipe("FOO".to_string()).to_string(), "foo");
    }

    #[test]
    fn single_char() {
        assert_eq!(Snake.pipe("A".to_string()).to_string(), "a");
    }

    #[test]
    fn trailing_uppercase() {
        assert_eq!(Snake.pipe("fooBAR".to_string()).to_string(), "foo_bar");
    }

    #[test]
    fn leading_underscores_stripped() {
        assert_eq!(Snake.pipe("__foo".to_string()).to_string(), "foo");
    }
}

mod pascal {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(Pascal.pipe("foo".to_string()).to_string(), "Foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(Pascal.pipe("foo_bar".to_string()).to_string(), "FooBar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(Pascal.pipe("fooBar".to_string()).to_string(), "FooBar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(Pascal.pipe("FooBar".to_string()).to_string(), "FooBar");
    }

    #[test]
    fn consecutive_uppercase() {
        assert_eq!(
            Pascal.pipe("http_response".to_string()).to_string(),
            "HttpResponse"
        );
    }

    #[test]
    fn all_uppercase_word() {
        assert_eq!(Pascal.pipe("FOO".to_string()).to_string(), "Foo");
    }

    #[test]
    fn single_char() {
        assert_eq!(Pascal.pipe("a".to_string()).to_string(), "A");
    }

    #[test]
    fn leading_underscores() {
        assert_eq!(Pascal.pipe("__foo".to_string()).to_string(), "Foo");
    }

    #[test]
    fn from_screaming() {
        assert_eq!(Pascal.pipe("FOO_BAR".to_string()).to_string(), "FooBar");
    }
}

mod camel {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(Camel.pipe("foo".to_string()).to_string(), "foo");
    }

    #[test]
    fn from_snake() {
        assert_eq!(Camel.pipe("foo_bar".to_string()).to_string(), "fooBar");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(Camel.pipe("FooBar".to_string()).to_string(), "fooBar");
    }

    #[test]
    fn from_camel() {
        assert_eq!(Camel.pipe("fooBar".to_string()).to_string(), "fooBar");
    }

    #[test]
    fn single_char() {
        assert_eq!(Camel.pipe("A".to_string()).to_string(), "a");
    }

    #[test]
    fn from_screaming() {
        assert_eq!(Camel.pipe("FOO_BAR".to_string()).to_string(), "fooBar");
    }
}

mod screaming {
    use super::*;

    #[test]
    fn from_lower() {
        assert_eq!(Screaming.pipe("foo".to_string()).to_string(), "FOO");
    }

    #[test]
    fn from_snake() {
        assert_eq!(Screaming.pipe("foo_bar".to_string()).to_string(), "FOO_BAR");
    }

    #[test]
    fn from_camel() {
        assert_eq!(Screaming.pipe("fooBar".to_string()).to_string(), "FOO_BAR");
    }

    #[test]
    fn from_pascal() {
        assert_eq!(Screaming.pipe("FooBar".to_string()).to_string(), "FOO_BAR");
    }

    #[test]
    fn consecutive_uppercase() {
        assert_eq!(
            Screaming.pipe("HTTPResponse".to_string()).to_string(),
            "HTTP_RESPONSE"
        );
    }
}

mod kebab {
    use super::*;

    #[test]
    fn from_pascal() {
        assert_eq!(Kebab.pipe("FooBar".to_string()).value(), "foo-bar");
    }

    #[test]
    fn from_snake() {
        assert_eq!(Kebab.pipe("foo_bar".to_string()).value(), "foo-bar");
    }

    #[test]
    fn from_screaming() {
        assert_eq!(Kebab.pipe("FOO_BAR".to_string()).value(), "foo-bar");
    }

    #[test]
    fn consecutive_uppercase() {
        assert_eq!(
            Kebab.pipe("HTTPResponse".to_string()).value(),
            "http-response"
        );
    }
}

mod case_functions {
    use zyn_core::case;

    #[test]
    fn snake_from_pascal() {
        assert_eq!(case::to_snake("HelloWorld"), "hello_world");
    }

    #[test]
    fn snake_consecutive_uppercase() {
        assert_eq!(case::to_snake("HTTPResponse"), "http_response");
    }

    #[test]
    fn snake_all_uppercase() {
        assert_eq!(case::to_snake("FOO"), "foo");
    }

    #[test]
    fn pascal_from_snake() {
        assert_eq!(case::to_pascal("hello_world"), "HelloWorld");
    }

    #[test]
    fn pascal_from_camel() {
        assert_eq!(case::to_pascal("fooBar"), "FooBar");
    }

    #[test]
    fn pascal_from_screaming() {
        assert_eq!(case::to_pascal("FOO_BAR"), "FooBar");
    }

    #[test]
    fn camel_from_snake() {
        assert_eq!(case::to_camel("hello_world"), "helloWorld");
    }

    #[test]
    fn camel_from_pascal() {
        assert_eq!(case::to_camel("FooBar"), "fooBar");
    }

    #[test]
    fn screaming_from_pascal() {
        assert_eq!(case::to_screaming("HelloWorld"), "HELLO_WORLD");
    }

    #[test]
    fn screaming_consecutive_uppercase() {
        assert_eq!(case::to_screaming("HTTPResponse"), "HTTP_RESPONSE");
    }

    #[test]
    fn kebab_from_pascal() {
        assert_eq!(case::to_kebab("HelloWorld"), "hello-world");
    }

    #[test]
    fn kebab_consecutive_uppercase() {
        assert_eq!(case::to_kebab("HTTPResponse"), "http-response");
    }
}
