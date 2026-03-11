use zyn::path::{MetaPath, Segment};

#[test]
fn parse_single_key() {
    let path = MetaPath::parse("serde").unwrap();
    assert_eq!(path.len(), 1);
    assert_eq!(path.first().unwrap().as_key(), Some("serde"));
}

#[test]
fn parse_dotted_keys() {
    let path = MetaPath::parse("serde.rename").unwrap();
    assert_eq!(path.len(), 2);

    let segs: Vec<_> = path.segments().iter().collect();
    assert_eq!(segs[0].as_key(), Some("serde"));
    assert_eq!(segs[1].as_key(), Some("rename"));
}

#[test]
fn parse_index() {
    let path = MetaPath::parse("derive[0]").unwrap();
    assert_eq!(path.len(), 2);
    assert_eq!(path.segments()[0].as_key(), Some("derive"));
    assert_eq!(path.segments()[1].as_index(), Some(0));
}

#[test]
fn parse_mixed() {
    let path = MetaPath::parse("a.b[2].c").unwrap();
    assert_eq!(path.len(), 4);
    assert_eq!(path.segments()[0].as_key(), Some("a"));
    assert_eq!(path.segments()[1].as_key(), Some("b"));
    assert_eq!(path.segments()[2].as_index(), Some(2));
    assert_eq!(path.segments()[3].as_key(), Some("c"));
}

#[test]
fn tail_removes_first() {
    let path = MetaPath::parse("a.b.c").unwrap();
    let tail = path.tail();
    assert_eq!(tail.len(), 2);
    assert_eq!(tail.segments()[0].as_key(), Some("b"));
    assert_eq!(tail.segments()[1].as_key(), Some("c"));
}

#[test]
fn display_round_trip() {
    let input = "serde.rename";
    let path = MetaPath::parse(input).unwrap();
    assert_eq!(path.to_string(), input);
}

#[test]
fn display_round_trip_with_index() {
    let input = "derive[0].name";
    let path = MetaPath::parse(input).unwrap();
    assert_eq!(path.to_string(), input);
}

#[test]
fn parse_empty_is_error() {
    assert!(MetaPath::parse("").is_err());
}

#[test]
fn segment_predicates() {
    let key = Segment::from("name");
    assert!(key.is_key());
    assert!(!key.is_index());

    let idx = Segment::from(0usize);
    assert!(idx.is_index());
    assert!(!idx.is_key());
}
