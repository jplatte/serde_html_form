//! Deserialization support for the `application/x-www-form-urlencoded` format.

use form_urlencoded::{parse, Parse as UrlEncodedParse};
use indexmap::map::{self, IndexMap};
use serde_core::{
    de::{self, value::MapDeserializer},
    forward_to_deserialize_any,
};

#[doc(inline)]
pub use serde_core::de::value::Error;

mod part;
mod val_or_vec;

use self::{part::Part, val_or_vec::ValOrVec};

/// Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_html_form::from_bytes::<Vec<(String, String)>>(
///         b"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"
///     ),
///     Ok(meal)
/// );
/// ```
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    T::deserialize(Deserializer::<false>::from_bytes(input))
}

/// Deserializes a `application/x-www-form-urlencoded` value from a `&str`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_html_form::from_str::<Vec<(String, String)>>(
///         "bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"
///     ),
///     Ok(meal)
/// );
/// ```
pub fn from_str<'de, T>(input: &'de str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    from_bytes(input.as_bytes())
}

/// A deserializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level outputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Main `deserialize` methods defers to `deserialize_map`.
///
/// * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
///   defers to `deserialize`.
pub struct Deserializer<'de, const EMPTY_IS_SOME: bool = false> {
    inner: UrlEncodedParse<'de>,
}

impl<'de, const EMPTY_IS_SOME: bool> Deserializer<'de, EMPTY_IS_SOME> {
    /// Returns a new `Deserializer`.
    pub fn new(parse: UrlEncodedParse<'de>) -> Self {
        Deserializer { inner: parse }
    }

    /// Returns a new `Deserializer` from a `&[u8]`.
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Self::new(parse(input))
    }
}

impl<'de, const EMPTY_IS_SOME: bool> de::Deserializer<'de> for Deserializer<'de, EMPTY_IS_SOME> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor
            .visit_map(MapDeserializer::new(group_entries::<EMPTY_IS_SOME>(self.inner).into_iter()))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(MapDeserializer::new(PartIterator::<EMPTY_IS_SOME>(self.inner)))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let deserializer = MapDeserializer::new(PartIterator::<EMPTY_IS_SOME>(self.inner));
        deserializer.end()?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        option
        bytes
        byte_buf
        unit_struct
        tuple_struct
        struct
        identifier
        tuple
        enum
        ignored_any
    }
}

struct PartIterator<'de, const EMPTY_IS_SOME: bool>(UrlEncodedParse<'de>);

impl<'de, const EMPTY_IS_SOME: bool> Iterator for PartIterator<'de, EMPTY_IS_SOME> {
    type Item = (Part<'de, EMPTY_IS_SOME>, Part<'de, EMPTY_IS_SOME>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (Part(k), Part(v)))
    }
}

fn group_entries<const EMPTY_IS_SOME: bool>(
    parse: UrlEncodedParse<'_>,
) -> IndexMap<Part<'_, EMPTY_IS_SOME>, ValOrVec<Part<'_, EMPTY_IS_SOME>>> {
    use map::Entry::*;

    let mut res = IndexMap::new();

    for (key, value) in parse {
        match res.entry(Part(key)) {
            Vacant(v) => {
                v.insert(ValOrVec::Val(Part(value)));
            }
            Occupied(mut o) => {
                o.get_mut().push(Part(value));
            }
        }
    }

    res
}

/// Deserialization with empty strings treated as `Some("")` instead of `None`.
///
/// This module provides the same API as the parent module, but with different
/// behavior for empty values: `foo=` deserializes to `Some("")` for `Option<String>`
/// instead of `None`.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct Form {
///     value: Option<String>,
/// }
///
/// // Default behavior: empty string becomes None
/// assert_eq!(
///     serde_html_form::from_str::<Form>("value="),
///     Ok(Form { value: None })
/// );
///
/// // With empty_is_some: empty string becomes Some("")
/// assert_eq!(
///     serde_html_form::empty_is_some::from_str::<Form>("value="),
///     Ok(Form { value: Some("".to_owned()) })
/// );
/// ```
pub mod empty_is_some {
    use super::{de, Error};

    /// A type alias for the deserializer with `EMPTY_IS_SOME = true`.
    pub type Deserializer<'de> = super::Deserializer<'de, true>;

    /// Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.
    ///
    /// Empty values are deserialized as `Some("")` for `Option<String>` fields.
    pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, Error>
    where
        T: de::Deserialize<'de>,
    {
        T::deserialize(Deserializer::from_bytes(input))
    }

    /// Deserializes a `application/x-www-form-urlencoded` value from a `&str`.
    ///
    /// Empty values are deserialized as `Some("")` for `Option<String>` fields.
    pub fn from_str<'de, T>(input: &'de str) -> Result<T, Error>
    where
        T: de::Deserialize<'de>,
    {
        from_bytes(input.as_bytes())
    }
}

#[cfg(test)]
mod tests;
