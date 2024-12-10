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

use crate::control::FileEntry;
use std::ops::Deref;

/// List of [FileEntry] traited structs which contain a description of a
/// specific file contained in a [crate::control::changes::Changes] upload.
#[derive(Clone, Debug, PartialEq)]
pub struct Files<FileT>(pub Vec<FileT>)
where
    FileT: FileEntry;

impl<FileT> Deref for Files<FileT>
where
    FileT: FileEntry,
{
    type Target = [FileT];

    fn deref(&self) -> &[FileT] {
        self.0.as_ref()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::{FileEntry, Files};
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    impl<FileT> Serialize for Files<FileT>
    where
        FileT: ToString,
        FileT: FileEntry,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(
                &format!(
                    "\n{}",
                    self.0
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                serializer,
            )
        }
    }

    impl<'de, FileT> Deserialize<'de> for Files<FileT>
    where
        FileT: FileEntry,
        FileT: FromStr,
        FileT::Err: std::fmt::Debug,
    {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            let mut files = vec![];
            for line in s.trim_start().split('\n') {
                files.push(
                    line.parse()
                        .map_err(|e| D::Error::custom(format!("{:?}", e)))?,
                );
            }
            Ok(Self(files))
        }
    }
}

// vim: foldmethod=marker
