# `serde_html_form`

(De-)serialization support for the `application/x-www-form-urlencoded` format.

This crate is a Rust library for serialising to and deserialising from
the [`application/x-www-form-urlencoded`][urlencoded] format. It is built
upon [Serde], a high performance generic serialization framework and [rust-url],
a URL parser for Rust.

[rust-url]: https://github.com/servo/rust-url
[Serde]: https://github.com/serde-rs/serde
[urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded

## Installation

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
serde_html_form = "0.1.0"
```

The documentation is available on [docs.rs].

[crates.io]: https://crates.io/crates/serde_html_form
[docs.rs]: https://docs.rs/serde_html_form

## License

This crate is licensed under the MIT license ([LICENSE](LICENSE) or
http://opensource.org/licenses/MIT).
