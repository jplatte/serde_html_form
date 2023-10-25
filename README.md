# `serde_html_form`

(De-)serialization support for the `application/x-www-form-urlencoded` format.

This crate is a Rust library for serialising to and deserialising from
the [`application/x-www-form-urlencoded`][urlencoded] format. It is built
upon [Serde], a high performance generic serialization framework and [rust-url],
a URL parser for Rust.

It is a fork of [`serde_urlencoded`], with additional support for maps or
structs with fields of sequence type (e.g. `Vec<String>`). It also supports
`Option` in values, treating `foo=` as `foo: None`.

⚠️ For vectors that may be empty, it is best to use a `Vec` with the
`#[serde(default)]` attribute, as the combination `Option<Vec<_>>` can lead
to unexpected behavior when deserializing a `Vec` with exactly one value.

[rust-url]: https://github.com/servo/rust-url
[Serde]: https://github.com/serde-rs/serde
[urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
[`serde_urlencoded`]: https://github.com/nox/serde_urlencoded

## Installation

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
serde_html_form = "0.2.0"
```

The documentation is available on [docs.rs].

[crates.io]: https://crates.io/crates/serde_html_form
[docs.rs]: https://docs.rs/serde_html_form

## License

This crate is licensed under the MIT license ([LICENSE](LICENSE) or
http://opensource.org/licenses/MIT).
