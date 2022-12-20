# 0.1.1

Support deserialization of `Option`al values to better support forms with optional inputs of non-string types:

```rust
#[derive(Deserialize, PartialEq)]
struct MyForm {
    field: Option<u16>,
}

// What browsers send when a value is given
assert_eq!(serde_html_form::from_str("field=5").unwrap(), MyForm { field: Some(5) });
// What browsers send when no value is given
assert_eq!(serde_html_form::from_str("field=").unwrap(), MyForm { field: None });
// This also works
assert_eq!(serde_html_form::from_str("").unwrap(), MyForm { field: None });
```

# 0.1.0

Initial release.
