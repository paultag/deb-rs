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

macro_rules! def_serde_traits_for {
    ($type:ident) => {
        #[cfg(feature = "serde")]
        mod serde {
            use super::$type;
            use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

            impl Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    String::serialize(&self.to_string(), serializer)
                }
            }

            impl<'de> Deserialize<'de> for $type {
                fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                    let s = String::deserialize(d)?;
                    s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
                }
            }
        }
    };
}
pub(super) use def_serde_traits_for;

#[cfg(test)]
macro_rules! def_parse_test {
    ($name:ident, $type:ty, $from:expr, $compare:expr) => {
        #[test]
        fn $name() {
            let v: $type = $from.parse().unwrap();
            assert_eq!($compare, v);
        }
    };
}
#[cfg(test)]
pub(super) use def_parse_test;

#[cfg(test)]
macro_rules! def_failing_parse_test {
    ($name:ident, $type:ty, $from:expr) => {
        #[test]
        fn $name() {
            assert!($from.parse::<$type>().is_err());
        }
    };
}
#[cfg(test)]
pub(super) use def_failing_parse_test;

// vim: foldmethod=marker
