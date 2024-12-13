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

use super::{ChangesParseError, HASH_LEN_MD5};
use crate::control::FileEntry;
use std::{path::PathBuf, str::FromStr};

/// [File] is a specific File  referenced by the
/// [crate::control::changes::Changes] file.
///
/// This entry contains a now very antiquated `md5` digest, which should
/// _not_ be used for basically any purpose.
#[derive(Clone, Debug, PartialEq)]
pub struct File {
    /// MD5 hash digest of a File contained in this upload.
    pub digest: String,

    /// File size, in bytes, of the File contained in this upload.
    pub size: usize,

    /// Path of the file relative to the Changes file.
    pub path: String,

    /// Section of the archive the file is targeted for.
    pub section: String,

    /// Priority of the file.
    pub priority: String,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} {} {} {} {}",
            self.digest, self.size, self.path, self.section, self.priority
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

        // It's ASCII encoded, which means every byte is two bytes. We're
        // going to handle conversion elsewhere since it's going to require
        // an external crate, which should be optional. For now we're going
        // to make sure the string *looks* passable.
        if digest.len() != (HASH_LEN_MD5 * 2) {
            return Err(ChangesParseError::InvalidHashLength);
        }

        Ok(File {
            digest: digest.to_owned(),
            size: size.parse().map_err(|_| ChangesParseError::Malformed)?,
            section: section.to_owned(),
            priority: priority.to_owned(),
            path: path.to_owned(),
        })
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::File;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for File {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for File {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

#[cfg(feature = "hex")]
mod hex {
    #![cfg_attr(docsrs, doc(cfg(feature = "hex")))]

    use super::*;
    use ::hex;

    impl File {
        /// Return the parsed digest for this File.
        pub fn digest(&self) -> Result<[u8; HASH_LEN_MD5], ChangesParseError> {
            hex::decode(self.ascii_digest().ok_or(ChangesParseError::InvalidHash)?)
                .map_err(|_| ChangesParseError::InvalidHash)?
                .try_into()
                .map_err(|_| ChangesParseError::InvalidHashLength)
        }
    }

    #[test]
    fn hex_digest_file() {
        let file = File {
            digest: "e7bd195571b19d33bd83d1c379fe6432".to_owned(),
            size: 1183,
            path: "hello_2.10-3.dsc".to_owned(),
            section: "devel".to_owned(),
            priority: "optional".to_owned(),
        };

        assert_eq!(
            hex::decode("e7bd195571b19d33bd83d1c379fe6432").unwrap(),
            file.digest().unwrap()
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
    fn ascii_digest(&self) -> Option<&str> {
        Some(&self.digest)
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

        assert_eq!("optional", file.priority);
        assert_eq!(12688, file.size);
    }
}

// vim: foldmethod=marker
