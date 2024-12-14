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

use crate::{
    architecture::{self, Architecture},
    control::def_serde_traits_for,
};
use std::{ops::Deref, str::FromStr};

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

impl std::fmt::Display for Architectures {
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

impl FromStr for Architectures {
    type Err = architecture::Error;

    fn from_str(architectures: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            architectures
                .split(' ')
                .map(|arch| arch.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

def_serde_traits_for!(Architectures);

// vim: foldmethod=marker
