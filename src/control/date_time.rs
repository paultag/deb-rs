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

use std::str::FromStr;

#[cfg(not(feature = "chrono"))]
type InnerDateTime = String;

#[cfg(feature = "chrono")]
type InnerDateTime = ::chrono::DateTime<::chrono::FixedOffset>;

/// Wrapper type around a `String` which optionally contains a helper to
/// convert to a [::chrono::DateTime] object if the `chrono` feature flag
/// is enabled.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct DateTime2822(InnerDateTime);

/// Error conditions which may be encountered when working with a
/// [DateTime2822].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DateTime2822ParseError {
    #[cfg(feature = "chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
    /// The provided [DateTime2822] had an invalid string representation of
    /// a date time.
    InvalidDate(::chrono::ParseError),
}
crate::errors::error_enum!(DateTime2822ParseError);

#[cfg(not(feature = "chrono"))]
mod not_chrono {
    use super::*;

    impl FromStr for DateTime2822 {
        type Err = DateTime2822ParseError;
        fn from_str(when: &str) -> Result<Self, Self::Err> {
            Ok(Self(when.to_owned()))
        }
    }

    impl std::fmt::Display for DateTime2822 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(feature = "chrono")]
mod chrono {

    use super::*;
    use ::chrono::{DateTime, FixedOffset};

    impl FromStr for DateTime2822 {
        type Err = DateTime2822ParseError;
        fn from_str(when: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                DateTime::parse_from_rfc2822(when).map_err(DateTime2822ParseError::InvalidDate)?,
            ))
        }
    }

    impl std::fmt::Display for DateTime2822 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }

    impl DateTime2822 {
        /// Returned the parsed [DateTime] for use.
        #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
        pub fn to_datetime(&self) -> &DateTime<FixedOffset> {
            &self.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_date_time_chrono_parse() {
            let _: DateTime2822 = "Mon, 26 Dec 2022 16:30:00 +0100".parse().unwrap();
        }

        #[test]
        fn test_date_time_chrono_wont_parse() {
            assert!("Tue, 26 Dec 2022 16:30:00 +0100"
                .parse::<DateTime2822>()
                .is_err());
        }
    }
}

super::def_serde_traits_for!(DateTime2822);

// vim: foldmethod=marker
