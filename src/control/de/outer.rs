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

use super::{paragraph, Error};
use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any,
};
use std::iter::Peekable;

pub(super) struct Deserializer<'a, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Iterator<Item = &'a str>,
    IteratorT: Clone,
{
    pub(super) iter: Peekable<IteratorT>,
}

impl<'a, 'de, IteratorT> de::Deserializer<'de> for &mut Deserializer<'a, IteratorT>
where
    IteratorT: 'a,
    IteratorT: Clone,
    IteratorT: Iterator<Item = &'a str>,
{
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        let mut de = paragraph::Deserializer {
            iter: self.iter.clone(),
        };
        de.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut de = paragraph::Deserializer {
            iter: self.iter.clone(),
        };
        de.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        char unit
        u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
        bytes byte_buf str string
        seq
        identifier bool option
        tuple unit_struct tuple_struct enum newtype_struct struct
        ignored_any
    }
}

// vim: foldmethod=marker
