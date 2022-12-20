use std::{hint::unreachable_unchecked, iter, mem, vec};

use crate::de::part::Part;
use serde::de::{
    self,
    value::{Error, SeqDeserializer},
    Deserializer, IntoDeserializer,
};

#[derive(Debug)]
pub enum ValOrVec<T> {
    Val(T),
    Vec(Vec<T>),
}

impl<T> ValOrVec<T> {
    pub fn push(&mut self, new_val: T) {
        match self {
            Self::Val(_) => {
                // Change self to a Vec variant and take ownership of the previous value
                let old_self = mem::replace(self, ValOrVec::Vec(Vec::with_capacity(2)));

                let old_val = match old_self {
                    Self::Val(v) => v,
                    // Safety: We would not be in the outer branch otherwise
                    _ => unsafe { unreachable_unchecked() },
                };

                let vec = match self {
                    ValOrVec::Vec(v) => v,
                    // Safety: We set self to Vec with the mem::replace above
                    _ => unsafe { unreachable_unchecked() },
                };

                vec.push(old_val);
                vec.push(new_val);
            }
            Self::Vec(vec) => vec.push(new_val),
        }
    }

    fn deserialize_val<U, E, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
        E: de::Error,
    {
        match self {
            ValOrVec::Val(val) => f(val),
            ValOrVec::Vec(_) => Err(de::Error::custom("unsupported value")),
        }
    }
}

impl<T> IntoIterator for ValOrVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub enum IntoIter<T> {
    Val(iter::Once<T>),
    Vec(vec::IntoIter<T>),
}

impl<T> IntoIter<T> {
    fn new(vv: ValOrVec<T>) -> Self {
        match vv {
            ValOrVec::Val(val) => IntoIter::Val(iter::once(val)),
            ValOrVec::Vec(vec) => IntoIter::Vec(vec.into_iter()),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Val(iter) => iter.next(),
            IntoIter::Vec(iter) => iter.next(),
        }
    }
}

impl<'de, T> IntoDeserializer<'de> for ValOrVec<T>
where
    T: IntoDeserializer<'de> + Deserializer<'de, Error = Error> + IsEmpty,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_to_part {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                self.deserialize_val(move |val| val.$method(visitor))
            }
        )*
    }
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Part<'_> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de, T> Deserializer<'de> for ValOrVec<T>
where
    T: IntoDeserializer<'de> + Deserializer<'de, Error = Error> + IsEmpty,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Self::Val(val) => val.deserialize_any(visitor),
            Self::Vec(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(self.into_iter()))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_enum(name, variants, visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_tuple(len, visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_struct(name, fields, visitor))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_unit_struct(name, visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_tuple_struct(name, len, visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_newtype_struct(name, visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let should_visit_none = match &self {
            ValOrVec::Val(val) if val.is_empty() => true,
            _ => false,
        };

        if should_visit_none {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    forward_to_part! {
        deserialize_bool,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_unit,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_f32,
        deserialize_f64,
        deserialize_identifier,
        deserialize_map,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use matches::assert_matches;

    use super::ValOrVec;

    #[test]
    fn cow_borrowed() {
        let mut x = ValOrVec::Val(Cow::Borrowed("a"));
        x.push(Cow::Borrowed("b"));
        x.push(Cow::Borrowed("c"));
        assert_matches!(x, ValOrVec::Vec(v) if v == vec!["a", "b", "c"]);
    }

    #[test]
    fn cow_owned() {
        let mut x = ValOrVec::Val(Cow::from("a".to_owned()));
        x.push(Cow::from("b".to_owned()));
        x.push(Cow::from("c".to_owned()));
        assert_matches!(
            x,
            ValOrVec::Vec(v) if v == vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
        );
    }
}
