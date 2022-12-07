use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct NewType<T>(T);

#[test]
fn deserialize_newtype_i32() {
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(super::from_str("field=11"), Ok(result));
}

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(super::from_bytes(b"first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(super::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(super::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(super::from_reader(b"first=23&last=42" as &[_]), Ok(result));
}

#[test]
fn deserialize_option() {
    let result = vec![("first".to_owned(), Some(23)), ("last".to_owned(), Some(42))];
    assert_eq!(super::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_option_vec() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<String>>,
    }

    assert_eq!(
        super::from_str("value=abc&value=def"),
        Ok(Form { value: Some(vec!["abc".to_owned(), "def".to_owned()]) })
    );
    assert_eq!(super::from_str(""), Ok(Form { value: None }));
}

#[test]
fn deserialize_unit() {
    assert_eq!(super::from_str(""), Ok(()));
    assert_eq!(super::from_str("&"), Ok(()));
    assert_eq!(super::from_str("&&"), Ok(()));
    assert!(super::from_str::<()>("first=23").is_err());
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum X {
    A,
    B,
    C,
}

#[test]
fn deserialize_unit_enum() {
    let result =
        vec![("one".to_owned(), X::A), ("two".to_owned(), X::B), ("three".to_owned(), X::C)];

    assert_eq!(super::from_str("one=A&two=B&three=C"), Ok(result));
}

#[test]
fn deserialize_unit_type() {
    assert_eq!(super::from_str(""), Ok(()));
}
