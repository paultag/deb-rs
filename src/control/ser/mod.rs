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

//! Module to use the [serde] serialization framework to encode data to
//! Debian's RFC2822-like format from Rust types.

#![cfg_attr(docsrs, doc(cfg(feature = "serde")))]

use serde::Serialize;

mod paragraph;

use paragraph::Serializer;

/// Error states returned by the serializer
#[derive(Debug)]
pub enum Error {
    /// Serde Ser error
    Ser(String),

    /// Bad type
    BadType,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "{:?}", self)
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(err: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Ser(err.to_string())
    }
}

/// Encode the provided value to a Debian RFC 2822 style stanza.
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)?;
    Ok(serializer.output())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::Version;

    #[derive(Clone, Debug, PartialEq, Serialize)]
    struct TestControlFile {
        #[serde(rename = "Package")]
        package: String,

        #[serde(rename = "Foo")]
        foo: String,

        #[serde(rename = "True-False")]
        true_false: bool,

        #[serde(rename = "X-A-Number")]
        a_number: usize,

        #[serde(rename = "Ello")]
        ello: Option<String>,
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            to_string(&TestControlFile {
                package: "foo".to_owned(),
                foo: "bar".to_owned(),
                true_false: true,
                a_number: 20,
                ello: Some("Foo".to_owned()),
            })
            .unwrap(),
            "\
Package: foo
Foo: bar
True-False: true
X-A-Number: 20
Ello: Foo
"
        );
    }

    #[derive(Clone, Debug, PartialEq, Serialize)]
    struct TestControl {
        #[serde(rename = "Foo")]
        foo: String,
    }

    #[test]
    fn test_string_newlines() {
        assert_eq!(
            to_string(&TestControl {
                foo: "
foo"
                .to_owned()
            })
            .unwrap(),
            "\
Foo:
 foo
"
        );
    }

    #[test]
    fn test_string_empty_lines() {
        assert_eq!(
            to_string(&TestControl {
                foo: "
foo

bar"
                .to_owned()
            })
            .unwrap(),
            "\
Foo:
 foo
 .
 bar
"
        );
    }

    #[test]
    fn test_multiline_custom() {
        #[derive(Clone, Debug, PartialEq, Serialize)]
        struct Multiline {
            #[serde(rename = "Multiline")]
            multiline: Vec<Version>,
        }

        assert_eq!(
            to_string(&Multiline {
                multiline: ["1.0", "1.1", "1.2", "1.3", "1.4"]
                    .iter()
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .unwrap(),
            "\
Multiline:
 1.0
 1.1
 1.2
 1.3
 1.4
",
        );
    }
}

// vim: foldmethod=marker
