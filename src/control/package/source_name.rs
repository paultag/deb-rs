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

use crate::{control::def_serde_traits_for, version::Version};
use std::str::FromStr;

/// Source package name. This is used in places where a control file needs to
/// refer back to a source package. If the version is specified, it means that
/// the binary control files' version isn't the same as the source version it
/// was built from.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceName {
    /// Name of the source package.
    pub name: String,

    /// Version of the package that the binary was generated from.
    pub version: Option<Version>,
}

def_serde_traits_for!(SourceName);

impl std::fmt::Display for SourceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.version {
            Some(version) => write!(f, "{} ({})", self.name, version),
            None => write!(f, "{}", self.name),
        }
    }
}

/// Errors encountered when parsing the Source package name.
///
/// In cases where a binary package don't have a version that matches the
/// Source package version, a trailing version will be attached to the source
/// package name.
#[derive(Copy, Clone, Debug)]
pub enum SourceNameError {
    /// Something was structured funny with the Source line
    Malformed,

    /// Version was malformed. Needs to be in the format of
    /// `SOURCE-NAME (VERSION)` if the version is specified.
    BadVersion,

    /// Source is empty, which is invalid.
    Empty,

    /// Underlying issue parsing the Version
    VersionError(crate::version::Error),
}

crate::errors::error_enum!(SourceNameError);

impl FromStr for SourceName {
    type Err = SourceNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SourceNameError::Empty);
        }

        if !s.contains(" ") {
            return Ok(Self {
                name: s.to_owned(),
                version: None,
            });
        }

        let [name, version] = s
            .splitn(2, " ")
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| SourceNameError::Malformed)?;

        if !version.starts_with("(") || !version.ends_with(")") {
            return Err(SourceNameError::BadVersion);
        }

        Ok(Self {
            name: name.to_owned(),
            version: Some(
                version[1..version.len() - 1]
                    .parse()
                    .map_err(SourceNameError::VersionError)?,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_normal() {
        let source: SourceName = "foo".parse().unwrap();
        assert_eq!("foo", source.name);
    }

    #[test]
    fn check_with_version() {
        let source: SourceName = "foo (1.0)".parse().unwrap();
        assert_eq!("foo", source.name);
        assert_eq!("1.0".parse::<Version>().unwrap(), source.version.unwrap());
    }

    macro_rules! check_fails {
        ($name:ident, $expr:expr ) => {
            #[test]
            fn $name() {
                assert!($expr.parse::<SourceName>().is_err());
            }
        };
    }

    check_fails!(bad_empty, "");
    check_fails!(bad_trailing, "foo ");
    check_fails!(bad_empty_version, "foo ()");
    check_fails!(bad_space, "foo ( )");
    check_fails!(bad_unmatched_begin, "foo )");
    check_fails!(bad_unmmatched_end, "foo (");
}

// vim: foldmethod=marker
