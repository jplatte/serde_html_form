use std::collections::BTreeMap;

use insta::{assert_compact_debug_snapshot, assert_snapshot};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct NewType<T>(T);

#[test]
fn deserialize_newtype_i32() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, NewType<i32>)>>("field=11"),
        @r#"Ok([("field", NewType(11))])"#
    );
}

#[test]
fn deserialize_bytes() {
    assert_compact_debug_snapshot!(
        super::from_bytes::<Vec<(String, i32)>>(b"first=23&last=42"),
        @r#"Ok([("first", 23), ("last", 42)])"#
    );
}

#[test]
fn deserialize_str() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, i32)>>("first=23&last=42"),
        @r#"Ok([("first", 23), ("last", 42)])"#
    );
}

#[test]
fn deserialize_borrowed_str() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(&str, i32)>>("first=23&last=42"),
        @r#"Ok([("first", 23), ("last", 42)])"#
    );
}

#[test]
fn deserialize_option() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, Option<i32>)>>("first=23&last=42"),
        @r#"Ok([("first", Some(23)), ("last", Some(42))])"#
    );
}

#[test]
fn deserialize_empty_string() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, &str)>>("first="),
        @r#"Ok([("first", "")])"#
    );
}

#[test]
fn deserialize_map() {
    assert_compact_debug_snapshot!(
        super::from_str::<BTreeMap<String, i32>>("first=23&second=42"),
        @r#"Ok({"first": 23, "second": 42})"#
    );
}

#[test]
fn deserialize_map_vec() {
    assert_compact_debug_snapshot!(
        super::from_str::<BTreeMap<String, Vec<i32>>>("first=23&second=42&first=1"),
        @r#"Ok({"first": [23, 1], "second": [42]})"#
    );
}

#[test]
fn deserialize_tuple_list() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, i32)>>("foo=1&bar=2&foo=3"),
        @r#"Ok([("foo", 1), ("bar", 2), ("foo", 3)])"#
    );
}

#[test]
fn deserialize_vec_strings() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<String>,
    }

    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=&value=abc"),
        @r#"Ok(Form { value: ["", "abc"] })"#
    );
}

#[test]
fn deserialize_option_vec() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<String>>,
    }

    assert_compact_debug_snapshot!(super::from_str::<Form>(""), @"Ok(Form { value: None })");
    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=abc"),
        @r#"Ok(Form { value: Some(["abc"]) })"#
    );
    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=abc&value=def"),
        @r#"Ok(Form { value: Some(["abc", "def"]) })"#
    );
}

#[test]
fn deserialize_option_vec_int() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<i32>>,
    }

    assert_compact_debug_snapshot!(super::from_str::<Form>(""), @"Ok(Form { value: None })");
    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=0"),
        @"Ok(Form { value: Some([0]) })"
    );
    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=3&value=-1"),
        @"Ok(Form { value: Some([3, -1]) })"
    );
}

#[test]
fn deserialize_option_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<f64>,
    }

    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value="),
        @"Ok(Form { value: None })"
    );
}

#[test]
fn deserialize_vec_options_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=&value=&value="),
        @"Ok(Form { value: [None, None, None] })"
    );
}

#[test]
fn deserialize_vec_options_some_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Vec<Option<f64>>,
    }

    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=&value=4&value="),
        @"Ok(Form { value: [None, Some(4.0), None] })"
    );
}

#[test]
fn deserialize_option_vec_no_value() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    assert_snapshot!(
        super::from_str::<Form>("value=&value=&value=").unwrap_err().to_string(),
        @"cannot parse float from empty string"
    );
}

#[test]
fn deserialize_option_vec_with_values() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: Option<Vec<f64>>,
    }

    assert_compact_debug_snapshot!(
        super::from_str::<Form>("value=3&value=4&value=5"),
        @"Ok(Form { value: Some([3.0, 4.0, 5.0]) })"
    );
}

#[test]
fn deserialize_no_value_err() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Form {
        value: f64,
    }

    assert_snapshot!(
        super::from_str::<Form>("value=").unwrap_err().to_string(),
        @"cannot parse float from empty string"
    );
}

#[test]
fn deserialize_unit() {
    assert_compact_debug_snapshot!(super::from_str::<()>(""), @"Ok(())");
    assert_compact_debug_snapshot!(super::from_str::<()>("&"), @"Ok(())");
    assert_compact_debug_snapshot!(super::from_str::<()>("&&"), @"Ok(())");
    assert_snapshot!(super::from_str::<()>("first=23").unwrap_err(), @"invalid length 1, expected 0 elements in map");
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum X {
    A,
    B,
    C,
}

#[test]
fn deserialize_unit_enum() {
    assert_compact_debug_snapshot!(
        super::from_str::<Vec<(String, X)>>("one=A&two=B&three=C"),
        @r#"Ok([("one", A), ("two", B), ("three", C)])"#
    );
}

#[test]
fn deserialize_unit_type() {
    assert_compact_debug_snapshot!(super::from_str::<()>(""), @"Ok(())");
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "lowercase")]
enum Foo {
    A(A),
}

#[derive(Debug, Deserialize, PartialEq)]
struct A {
    foo: String,
}

#[test]
fn deserialize_internally_tagged_enum() {
    assert_compact_debug_snapshot!(
        super::from_str::<Foo>("action=a&foo=hello"),
        @r#"Ok(A(A { foo: "hello" }))"#
    );
}
