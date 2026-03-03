use zyn_core::{Camel, Pascal, Pipe, Screaming, Snake};

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
}
