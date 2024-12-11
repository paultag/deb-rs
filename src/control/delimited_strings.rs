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

use std::ops::Deref;

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of space separated String values to and from a single String
/// as seen throughout the `control` module.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct SpaceDelimitedStrings(pub Vec<String>);

impl Deref for SpaceDelimitedStrings {
    type Target = [String];
    fn deref(&self) -> &[String] {
        &self.0
    }
}

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of comma separated String values to and from a single String
/// as seen throughout the `control` module.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CommaDelimitedStrings(pub Vec<String>);

impl Deref for CommaDelimitedStrings {
    type Target = [String];
    fn deref(&self) -> &[String] {
        &self.0
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::{CommaDelimitedStrings, SpaceDelimitedStrings};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for SpaceDelimitedStrings {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.0.to_vec().join(" "), serializer)
        }
    }

    impl<'de> Deserialize<'de> for SpaceDelimitedStrings {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            Ok(Self(s.split(' ').map(|v| v.to_owned()).collect::<Vec<_>>()))
        }
    }

    impl Serialize for CommaDelimitedStrings {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.0.to_vec().join(", "), serializer)
        }
    }

    impl<'de> Deserialize<'de> for CommaDelimitedStrings {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            Ok(Self(
                s.split(',')
                    .map(|v| v.trim().to_owned())
                    .collect::<Vec<_>>(),
            ))
        }
    }
}

// vim: foldmethod=marker