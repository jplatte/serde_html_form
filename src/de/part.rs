use alloc::borrow::Cow;
use core::any::TypeId;

use serde_core::{
    de::{self, Error as _, IntoDeserializer},
    forward_to_deserialize_any,
};

use crate::de::utils::non_static_type_id;

use super::Error;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(super) struct Part<'de>(pub Cow<'de, str>);

impl<'de> IntoDeserializer<'de> for Part<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_parsed_value {
    ($($ty:ident => $method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$method(visitor),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        )*
    }
}

impl<'de> de::Deserializer<'de> for Part<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_string(value),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.0.is_empty() {
            // Types for which to treat an empty `Part` as none in this method.
            //
            // FIXME: Change to a `const` once MSRV is raised to 1.91 or later.
            let empty_is_none_types = [
                TypeId::of::<Option<bool>>(),
                // signed integers
                TypeId::of::<Option<i8>>(),
                TypeId::of::<Option<i16>>(),
                TypeId::of::<Option<i32>>(),
                TypeId::of::<Option<i64>>(),
                TypeId::of::<Option<i128>>(),
                TypeId::of::<Option<isize>>(),
                TypeId::of::<Option<core::num::NonZeroI8>>(),
                TypeId::of::<Option<core::num::NonZeroI16>>(),
                TypeId::of::<Option<core::num::NonZeroI32>>(),
                TypeId::of::<Option<core::num::NonZeroI64>>(),
                TypeId::of::<Option<core::num::NonZeroI128>>(),
                TypeId::of::<Option<core::num::NonZeroIsize>>(),
                // unsigned integers
                TypeId::of::<Option<u8>>(),
                TypeId::of::<Option<u16>>(),
                TypeId::of::<Option<u32>>(),
                TypeId::of::<Option<u64>>(),
                TypeId::of::<Option<u128>>(),
                TypeId::of::<Option<usize>>(),
                TypeId::of::<Option<core::num::NonZeroU8>>(),
                TypeId::of::<Option<core::num::NonZeroU16>>(),
                TypeId::of::<Option<core::num::NonZeroU32>>(),
                TypeId::of::<Option<core::num::NonZeroU64>>(),
                TypeId::of::<Option<core::num::NonZeroU128>>(),
                TypeId::of::<Option<core::num::NonZeroUsize>>(),
                // floats
                TypeId::of::<Option<f32>>(),
                TypeId::of::<Option<f64>>(),
            ];

            let value_type = non_static_type_id::<V::Value>();
            if empty_is_none_types.contains(&value_type) {
                return visitor.visit_none();
            }
        }

        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(PartSeqAccess(Some(self)))
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        map
    }

    forward_parsed_value! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}

impl<'de> de::EnumAccess<'de> for Part<'de> {
    type Error = Error;
    type Variant = UnitOnlyVariantAccess;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.0.into_deserializer())?;
        Ok((variant, UnitOnlyVariantAccess))
    }
}

struct PartSeqAccess<'de>(Option<Part<'de>>);

impl<'de> de::SeqAccess<'de> for PartSeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.0.take() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.is_some() as usize)
    }
}

pub(crate) struct UnitOnlyVariantAccess;

impl<'de> de::VariantAccess<'de> for UnitOnlyVariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }
}
