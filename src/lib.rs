#![doc = include_str!("../README.md")]
#![no_std]
#![warn(
    rust_2018_idioms,
    unused_qualifications,
    clippy::alloc_instead_of_core,
    clippy::branches_sharing_code,
    clippy::cloned_instead_of_copied,
    clippy::empty_line_after_outer_attr,
    clippy::inefficient_to_string,
    clippy::mut_mut,
    clippy::nonstandard_macro_braces,
    clippy::semicolon_if_nothing_returned,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::{
    de::{from_bytes, from_str, Deserializer},
    ser::{push_to_string, to_string, Serializer},
};
