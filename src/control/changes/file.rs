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

use super::ChangesParseError;
use crate::control::{def_serde_traits_for, ChecksumMd5, FileEntry, Priority};
use std::{path::PathBuf, str::FromStr};

/// [File] is a specific File  referenced by the
/// [crate::control::changes::Changes] file.
///
/// This entry contains a now very antiquated `md5` digest, which should
/// _not_ be used for basically any purpose.
#[derive(Clone, Debug, PartialEq)]
pub struct File {
    /// MD5 hash digest of a File contained in this upload.
    pub digest: ChecksumMd5,

    /// File size, in bytes, of the File contained in this upload.
    pub size: usize,

    /// Path of the file relative to the Changes file.
    pub path: String,

    /// Section of the archive the file is targeted for.
    pub section: String,

    /// Priority of the file.
    pub priority: Option<Priority>,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} {} {} {} {}",
            self.digest,
            self.size,
            self.section,
            self.priority
                .map(|v| v.to_string())
                .unwrap_or("-".to_string()),
            self.path,
        )
    }
}

impl FromStr for File {
    type Err = ChangesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [digest, size, section, priority, path] = s
            .split(" ")
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ChangesParseError::Malformed)?;

        let priority: Option<Priority> = if priority != "-" {
            Some(
                priority
                    .parse()
                    .map_err(ChangesParseError::InvalidPriority)?,
            )
        } else {
            None
        };

        Ok(File {
            digest: digest.parse().map_err(|_| ChangesParseError::InvalidHash)?,
            size: size.parse().map_err(|_| ChangesParseError::Malformed)?,
            section: section.to_owned(),
            priority,
            path: path.to_owned(),
        })
    }
}

def_serde_traits_for!(File);

#[cfg(feature = "hex")]
mod hex {
    #![cfg_attr(docsrs, doc(cfg(feature = "hex")))]

    #[test]
    fn hex_digest_file() {
        use super::super::*;
        use crate::control::Priority;
        use ::hex;

        let file = File {
            digest: "e7bd195571b19d33bd83d1c379fe6432".parse().unwrap(),
            size: 1183,
            path: "hello_2.10-3.dsc".to_owned(),
            section: "devel".to_owned(),
            priority: Some(Priority::Optional),
        };

        assert_eq!(
            hex::decode("e7bd195571b19d33bd83d1c379fe6432").unwrap(),
            file.digest.digest(),
        );
    }
}

impl FileEntry for File {
    type Error = ChangesParseError;

    fn path(&self) -> Result<PathBuf, Self::Error> {
        Ok(self.path.parse().unwrap())
    }
    fn size(&self) -> Option<usize> {
        Some(self.size)
    }
    fn ascii_digest(&self) -> Option<String> {
        Some(self.digest.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let file: File =
            "16678389ba7fddcdfa05e0707d61f043 12688 devel optional hello_2.10-3.debian.tar.xz"
                .parse()
                .unwrap();

        assert_eq!(Priority::Optional, file.priority.unwrap());
        assert_eq!(12688, file.size);
    }
}

// vim: foldmethod=marker
