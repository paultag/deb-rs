// {{{ Copyright (c) Paul R. Tagliamonte <paultag@debian.org>, 2024
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE. }}}

use super::Error;
use serde::{
    de::{self, DeserializeSeed, Visitor},
    forward_to_deserialize_any,
};
use std::iter::Peekable;

/// Deserializer for deb822 style Control blocks.
pub(super) struct Deserializer<'a, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
{
    pub(super) iter: Peekable<IteratorT>,
}

impl<'a, IteratorT> Deserializer<'a, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
{
    fn next_float(&mut self) -> Result<f64, Error> {
        if let Some(next) = self.iter.next() {
            return next.parse().map_err(|_| Error::InvalidNumber);
        }
        Err(Error::EndOfFile)
    }

    fn next_number(&mut self) -> Result<i128, Error> {
        if let Some(next) = self.iter.next() {
            return next.parse().map_err(|_| Error::InvalidNumber);
        }
        Err(Error::EndOfFile)
    }
}

macro_rules! deserialize_float {
    ($name:ident, |$num:ident, $visitor:ident| $block:tt) => {
        fn $name<V>(self, $visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let $num = self.next_float()?;
            return $block;
        }
    };
}

macro_rules! deserialize_numerical {
    ($name:ident, |$num:ident, $visitor:ident| $block:tt) => {
        fn $name<V>(self, $visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let $num = self.next_number()?;
            return $block;
        }
    };
}

impl<'a, 'de, IteratorT> de::Deserializer<'de> for &mut Deserializer<'a, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
{
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        if let Some(next) = self.iter.next() {
            return visitor.visit_str(next);
        }
        Err(Error::EndOfFile)
    }

    forward_to_deserialize_any! {
        char unit
        bytes byte_buf str string
        seq
        tuple unit_struct tuple_struct enum newtype_struct
        ignored_any
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(next) = self.iter.next() {
            return visitor.visit_str(next);
        }
        Err(Error::EndOfFile)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        if let Some(next) = self.iter.next() {
            return visitor.visit_bool(match next.to_lowercase().as_str() {
                "true" => true,
                "false" => false,
                "yes" => true,
                "no" => false,
                _ => return Err(Error::InvalidBool),
            });
        }
        Err(Error::EndOfFile)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.iter.peek().is_some() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    deserialize_numerical!(deserialize_i8, |num, visitor| {
        visitor.visit_i8(num as i8)
    });
    deserialize_numerical!(deserialize_i16, |num, visitor| {
        visitor.visit_i16(num as i16)
    });
    deserialize_numerical!(deserialize_i32, |num, visitor| {
        visitor.visit_i32(num as i32)
    });
    deserialize_numerical!(deserialize_i64, |num, visitor| {
        visitor.visit_i64(num as i64)
    });
    deserialize_numerical!(deserialize_i128, |num, visitor| {
        // braces are needed
        visitor.visit_i128(num)
    });

    deserialize_numerical!(deserialize_u8, |num, visitor| {
        visitor.visit_u8(num as u8)
    });
    deserialize_numerical!(deserialize_u16, |num, visitor| {
        visitor.visit_u16(num as u16)
    });
    deserialize_numerical!(deserialize_u32, |num, visitor| {
        visitor.visit_u32(num as u32)
    });
    deserialize_numerical!(deserialize_u64, |num, visitor| {
        visitor.visit_u64(num as u64)
    });
    deserialize_numerical!(deserialize_u128, |num, visitor| {
        visitor.visit_u128(num as u128)
    });

    deserialize_float!(deserialize_f32, |num, visitor| {
        visitor.visit_f32(num as f32)
    });
    deserialize_float!(deserialize_f64, |num, visitor| {
        // braces needed
        visitor.visit_f64(num)
    });

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::BadType)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut uw = MapWrapper { de: self };
        visitor.visit_map(&mut uw)
    }
}

pub(super) struct MapWrapper<'a, 'b, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
{
    pub(super) de: &'b mut Deserializer<'a, IteratorT>,
}

impl<'a, 'b, 'de, IteratorT> de::MapAccess<'de> for &'b mut MapWrapper<'a, 'b, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.iter.peek().is_some() {
            return Ok(Some(seed.deserialize(&mut *self.de)?));
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

// vim: foldmethod=marker
