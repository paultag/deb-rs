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

use crate::architecture::Architecture;
use std::ops::Deref;

/// Wrapper type around a `Vec<Architecture>` which handles encoding and decoding
/// [Architecture] values to and from a String as seen throughout the `control`
/// module.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Architectures(pub Vec<Architecture>);

impl Deref for Architectures {
    type Target = [Architecture];
    fn deref(&self) -> &[Architecture] {
        self.0.as_ref()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Architectures;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Architectures {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(
                &self
                    .0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                serializer,
            )
        }
    }

    impl<'de> Deserialize<'de> for Architectures {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            Ok(Self(
                s.split(' ')
                    .map(|arch| {
                        arch.parse()
                            .map_err(|e| D::Error::custom(format!("{:?}", e)))
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
            ))
        }
    }
}

// vim: foldmethod=marker
