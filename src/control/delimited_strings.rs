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

use crate::control::def_serde_traits_for;
use std::{convert::Infallible, ops::Deref, str::FromStr};

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

mod space {
    use super::*;
    impl std::fmt::Display for SpaceDelimitedStrings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "{}",
                &self
                    .0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl FromStr for SpaceDelimitedStrings {
        type Err = Infallible;

        fn from_str(closes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                closes
                    .split(' ')
                    .map(|closes| closes.to_owned())
                    .collect::<Vec<_>>(),
            ))
        }
    }
    def_serde_traits_for!(SpaceDelimitedStrings);
}

mod comma {
    use super::*;
    impl std::fmt::Display for CommaDelimitedStrings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "{}",
                &self
                    .0
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl FromStr for CommaDelimitedStrings {
        type Err = Infallible;

        fn from_str(closes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                closes
                    .split(' ')
                    .map(|closes| closes.to_owned())
                    .collect::<Vec<_>>(),
            ))
        }
    }
    def_serde_traits_for!(CommaDelimitedStrings);
}

// vim: foldmethod=marker
