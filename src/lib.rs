#![doc = include_str!("../README.md")]
#![warn(
    rust_2018_idioms,
    unused_qualifications,
    clippy::branches_sharing_code,
    clippy::cloned_instead_of_copied,
    clippy::empty_line_after_outer_attr,
    clippy::inefficient_to_string,
    clippy::mut_mut,
    clippy::nonstandard_macro_braces,
    clippy::semicolon_if_nothing_returned,
    clippy::str_to_string,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]
#![deny(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::de::{from_bytes, from_str, Deserializer};

#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::de::from_reader;

#[doc(inline)]
pub use crate::ser::{push_to_string, to_string, Serializer};
