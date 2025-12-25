# Unreleased

- Change `deserialize_any` to forward to `deserialize_map` instead of `deserialize_seq`
  - This was a deviation from `serde_urlencoded`, which is now reverted
  - This means when deserializing to a catch-all type like `serde_json::Value`, you will now
    get an `Object` instead of an `Array`
- Change deserialization of optional values to treat empty values (like in `foo=&bar=`)
  as `Some(_)` rather than `None`, _except_ for `Option<bool>` and `Option<{number}>` (for
  specific number types that are either builtin or part of the standard library)
  - This reverts the main change from v0.1.1 while still allowing simple optional number fields to
    work
  - To get the old behavior for specific fields, use `#[serde(deserialize_with)]` with the new
    deserialization helper functions (see next changelog entry)
- Add `serde_html_form::de::empty_as_none` and `serde_html_form::de::empty_as_none::seq`
  - These allow treating empty values for a scalar field or sequence (list / set) field as `None`
- Remove `de::from_reader`

# 0.2.8

Switch `serde` dependency to `serde_core`.

# 0.2.7

Add `Deserializer::from_bytes`.

# 0.2.6

Fix deserialization of optional sequences of a single non-string element.

# 0.2.5

Add `push_to_string` for serializing a struct to the end of an existing `String`
buffer (instead of allocating a fresh one for the serialized output).

# 0.2.4

Fix deserialization of optional sequences of a single element.

# 0.2.3

Improve README and crate documentation (now the exact same, instead of just a
single-line description).

# 0.2.2

This release only upgrades one of the crates' dev-dependencies.

# 0.2.1

This release only upgrades one of the crates' private dependencies.

# 0.2.0

Support deserialization of sequences with duplicate keys.
This used to fail, but passes now:

```rust
let result = vec![("foo".to_owned(), 1), ("bar".to_owned(), 2), ("foo".to_owned(), 3)];
assert_eq!(super::from_str("foo=1&bar=2&foo=3"), Ok(result));
```

This should mainly affect deserialization to a type that's explicitly a sequence, like arrays or `Vec`,
but some other things were changed too so if you are getting unexpected deserialization errors, please open an issue.

This release has a minimum Rust version of 1.56.

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
