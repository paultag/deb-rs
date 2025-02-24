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

/// Generic type to hold a bunch of some parseable type delimited with
/// some char `DELIM`.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Delimited<const DELIM: char, InnerT>(pub Vec<InnerT>);

/// Repeated [String] values, seperated by `DELIM`.
pub type DelimitedStrings<const DELIM: char> = Delimited<DELIM, String>;

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of space separated String values to and from a single String
/// as seen throughout the `control` module.
pub type SpaceDelimitedStrings = DelimitedStrings<' '>;

/// Wrapper type around a `Vec<String>` which handles encoding and decoding
/// a list of comma separated String values to and from a single String
/// as seen throughout the `control` module.
pub type CommaDelimitedStrings = DelimitedStrings<','>;

impl<const DELIM: char, InnerT> Delimited<DELIM, InnerT> {
    /// Get the inner value as a ref
    pub fn get_ref(&self) -> &[InnerT] {
        &self.0
    }
}

impl<const DELIM: char, InnerT> Deref for Delimited<DELIM, InnerT> {
    type Target = [InnerT];
    fn deref(&self) -> &[InnerT] {
        &self.0
    }
}

impl<const DELIM: char, InnerT> std::fmt::Display for Delimited<DELIM, InnerT>
where
    InnerT: std::fmt::Display,
{
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

impl<const DELIM: char, InnerT> FromStr for Delimited<DELIM, InnerT>
where
    InnerT: FromStr,
    InnerT::Err: std::fmt::Debug,
{
    type Err = InnerT::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Ok(Self(vec![]));
        }

        Ok(Self(
            input
                .split(DELIM)
                .map(|closes| closes.parse::<InnerT>())
                .collect::<Result<Vec<InnerT>, _>>()?,
        ))
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Delimited;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    impl<const DELIM: char, InnerT> Serialize for Delimited<DELIM, InnerT>
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

    impl<'de, const DELIM: char, InnerT> Deserialize<'de> for Delimited<DELIM, InnerT>
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
    use super::{CommaDelimitedStrings, SpaceDelimitedStrings};
    use crate::control::{Delimited, def_parse_test};

    def_parse_test!(
        space_parse_empty,
        SpaceDelimitedStrings,
        "",
        Delimited::<' ', String>(vec![])
    );

    def_parse_test!(
        comma_parse_empty,
        CommaDelimitedStrings,
        "",
        Delimited::<',', String>(vec![])
    );

    def_parse_test!(
        space_parse_easy,
        SpaceDelimitedStrings,
        "foo bar",
        Delimited::<' ', String>(vec!["foo".to_owned(), "bar".to_owned()])
    );

    def_parse_test!(
        comma_parse_easy,
        CommaDelimitedStrings,
        "foo,bar",
        Delimited::<',', String>(vec!["foo".to_owned(), "bar".to_owned()])
    );

    def_parse_test!(
        space_parse_extra,
        SpaceDelimitedStrings,
        "foo  bar",
        Delimited::<' ', String>(vec!["foo".to_owned(), "".to_owned(), "bar".to_owned()])
    );

    def_parse_test!(
        comma_parse_extra,
        CommaDelimitedStrings,
        "foo,,bar",
        Delimited::<',', String>(vec!["foo".to_owned(), "".to_owned(), "bar".to_owned()])
    );
}

// vim: foldmethod=marker
