//! Extra deserialization helpers similar to [`empty_as_none`][fn@super::empty_as_none].

use alloc::{
    borrow::{Cow, ToOwned as _},
    string::String,
    vec::Vec,
};
use core::{fmt, marker::PhantomData};

use serde_core::de::{self, Deserialize, Unexpected, Visitor};

use super::Part;

/// Deserialization helper that treats empty values in a sequence as `None`.
///
/// Use with `#[serde(deserialize_with)]`. Do not use with deserializers from
/// other crates, as it may appear to work at first but result in strange
/// behavior later.
///
/// # Example
///
/// ```
/// # use std::collections::VecDeque;
/// # use serde::Deserialize;
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct Form {
///     #[serde(deserialize_with = "serde_html_form::de::empty_as_none::seq")]
///     values: VecDeque<Option<String>>,
/// }
///
/// let expected_values = VecDeque::from_iter([Some("x".to_owned()), None]);
/// assert_eq!(
///     serde_html_form::from_str("values=x&values="),
///     Ok(Form { values: expected_values }),
/// );
/// ```
pub fn seq<'de, L, T, D>(deserializer: D) -> Result<L, D::Error>
where
    L: Default + Extend<Option<T>> + IntoIterator<Item = Option<T>>,
    T: Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    let visitor = EmptyAsNoneListVisitor { _phantom: PhantomData };
    deserializer.deserialize_seq(visitor)
}

pub(super) struct EmptyAsNone<T>(pub Option<T>);

impl<'de, T> Deserialize<'de> for EmptyAsNone<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_core::Deserializer<'de>,
    {
        let s = deserializer.deserialize_string(CowStrVisitor)?;
        let v = if s.is_empty() {
            None
        } else {
            let value = T::deserialize(Part(s)).map_err(de::Error::custom)?;
            Some(value)
        };

        Ok(Self(v))
    }
}

struct CowStrVisitor;

impl<'de> Visitor<'de> for CowStrVisitor {
    type Value = Cow<'de, str>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::Borrowed(v))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match core::str::from_utf8(v) {
            Ok(s) => Ok(Cow::Borrowed(s)),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::Owned(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cow::Owned(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match core::str::from_utf8(v) {
            Ok(s) => Ok(Cow::Owned(s.to_owned())),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(Cow::Owned(s)),
            Err(e) => Err(de::Error::invalid_value(Unexpected::Bytes(&e.into_bytes()), &self)),
        }
    }
}

struct EmptyAsNoneListVisitor<L> {
    _phantom: PhantomData<L>,
}

impl<'de, L, T> Visitor<'de> for EmptyAsNoneListVisitor<L>
where
    L: Default + Extend<Option<T>> + IntoIterator<Item = Option<T>>,
    T: Deserialize<'de>,
{
    type Value = L;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut result = L::default();
        while let Some(EmptyAsNone(item)) = seq.next_element::<EmptyAsNone<T>>()? {
            result.extend([item]);
        }

        Ok(result)
    }
}
