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
/// a list of String values indicating bugs to be closed after the package
/// has been accepted.
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Closes(pub Vec<String>);

impl Deref for Closes {
    type Target = [String];
    fn deref(&self) -> &[String] {
        self.0.as_ref()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Closes;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Closes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.0.to_vec().join(" "), serializer)
        }
    }

    impl<'de> Deserialize<'de> for Closes {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            Ok(Self(
                s.split(' ').map(|arch| arch.to_owned()).collect::<Vec<_>>(),
            ))
        }
    }
}

// vim: foldmethod=marker
