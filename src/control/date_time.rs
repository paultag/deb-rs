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

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Wrapper type around a `String` which optionally contains a helper to
/// convert to a [::chrono::DateTime] object if the `chrono` feature flag
/// is enabled.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DateTime2822(pub String);

impl Deref for DateTime2822 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

/// Error conditions which may be encountered when working with a
/// [DateTime2822].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DateTime2822ParseError {
    #[cfg(feature = "chrono")]
    /// The provided [DateTime2822] had an invalid string represntation of
    /// a date time.
    InvalidDate,
}

#[cfg(feature = "chrono")]
mod chrono {
    use super::*;
    use ::chrono::{DateTime, FixedOffset};

    impl DateTime2822 {
        /// Return the date as a parsed [DateTime].
        ///
        /// # Note ♫
        ///
        /// This requires the `chrono` feature.
        pub fn as_chrono(&self) -> Result<DateTime<FixedOffset>, DateTime2822ParseError> {
            DateTime::parse_from_rfc2822(&self.0).map_err(|_| DateTime2822ParseError::InvalidDate)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_date_time_chrono_parse() {
            let dt = DateTime2822("Mon, 26 Dec 2022 16:30:00 +0100".to_owned());
            assert!(dt.as_chrono().is_ok());
        }
    }
}

// vim: foldmethod=marker
