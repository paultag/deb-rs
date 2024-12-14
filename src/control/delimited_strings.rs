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

use std::{convert::Infallible, ops::Deref, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct DelimitedStrings<const DELIM: char>(pub Vec<String>);

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of space separated String values to and from a single String
/// as seen throughout the `control` module.
pub type SpaceDelimitedStrings = DelimitedStrings<' '>;

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of comma separated String values to and from a single String
/// as seen throughout the `control` module.
pub type CommaDelimitedStrings = DelimitedStrings<','>;

impl<const DELIM: char> Deref for DelimitedStrings<DELIM> {
    type Target = [String];
    fn deref(&self) -> &[String] {
        &self.0
    }
}

impl<const DELIM: char> std::fmt::Display for DelimitedStrings<DELIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            &self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(&DELIM.to_string())
        )
    }
}

impl<const DELIM: char> FromStr for DelimitedStrings<DELIM> {
    type Err = Infallible;

    fn from_str(closes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            closes
                .split(DELIM)
                .map(|closes| closes.to_owned())
                .collect::<Vec<_>>(),
        ))
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::DelimitedStrings;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    impl<const DELIM: char> Serialize for DelimitedStrings<DELIM> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, const DELIM: char> Deserialize<'de> for DelimitedStrings<DELIM> {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

// vim: foldmethod=marker
