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

use std::{ops::Deref, str::FromStr};

/// Handle number parsing through a String to patch around a serde bug.
///
/// ⚠️  Avoid using this type if you can.
///
/// The Debian RFC 2822-like format is *not* "self describing". There's no
/// way to know the type of a value except to have knowledge of the type
/// for a given key.
///
/// There's a long-standing issue due to implementation choices made during
/// the creation of `flatten` that, when `flatten`ing structs,
/// the `serde` internals will dispatch to the `_any` helper for the values,
/// rather than the typed helpers, unlike for a normal deserialization.
///
/// I can't see any real useful workarounds and I don't think serde is going
/// to fix this anytime soon. As a result, I left the `_any` helper to call
/// back with `_str`; so flatten will work IFF all fiends are string-based
/// Deserilizations.
///
/// This type will deserialize it through a String rather than relying on
/// the actual Deserializer, since it's busted.
///
/// If on the long-shot chance that serde fixes their bug, this will likely
/// turn into a transparent type and marked deprecated. My money's not on
/// that happening, though.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Number<InnerT>(InnerT);

impl<InnerT> Deref for Number<InnerT> {
    type Target = InnerT;
    fn deref(&self) -> &InnerT {
        &self.0
    }
}

impl<InnerT> std::fmt::Display for Number<InnerT>
where
    InnerT: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.0)
    }
}

impl<InnerT> FromStr for Number<InnerT>
where
    InnerT: FromStr,
    InnerT::Err: std::fmt::Debug,
{
    type Err = InnerT::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.parse()?))
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Number;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    impl<InnerT> Serialize for Number<InnerT>
    where
        InnerT: std::fmt::Display,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, InnerT> Deserialize<'de> for Number<InnerT>
    where
        InnerT: std::str::FromStr,
        InnerT::Err: std::fmt::Debug,
    {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use crate::control::{def_failing_parse_test, def_parse_test};

    def_failing_parse_test!(fail_empty, Number<usize>, "");
    def_failing_parse_test!(fail_neg_one, Number<usize>, "-1");
    def_parse_test!(parse_zero, Number<usize>, "0", Number::<usize>(0));
    def_parse_test!(parse_one, Number<usize>, "1", Number::<usize>(1));
    def_parse_test!(parse_neg_one, Number<i32>, "-1", Number::<i32>(-1));
}

// vim: foldmethod=marker
