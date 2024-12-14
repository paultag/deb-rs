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
use crate::control::{Checksum, FileEntry};
use std::{path::PathBuf, str::FromStr};

/// [FileChecksum] is a specific File's hash digest and filesize referenced
/// by the [crate::control::changes::Changes] file.
#[derive(Clone, Debug, PartialEq)]
pub struct FileChecksum<const HASH_LEN: usize> {
    /// Hash digest of a File contained in this upload. The specific length
    /// of the digest is dicated by the hash algorithm.
    pub digest: Checksum<HASH_LEN>,

    /// File size, in bytes, of the File contained in this upload.
    pub size: usize,

    /// Path of the file relative to the Changes file.
    pub path: String,
}

impl<const HASH_LEN: usize> std::fmt::Display for FileChecksum<HASH_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.digest, self.size, self.path,)
    }
}

impl<const HASH_LEN: usize> FromStr for FileChecksum<HASH_LEN> {
    type Err = ChangesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [digest, size, path] = s
            .split(" ")
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ChangesParseError::Malformed)?;

        Ok(Self {
            digest: digest.parse().map_err(|_| ChangesParseError::InvalidHash)?,
            size: size.parse().map_err(|_| ChangesParseError::Malformed)?,
            path: path.to_owned(),
        })
    }
}

impl<const HASH_LEN: usize> FileEntry for FileChecksum<HASH_LEN> {
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

#[cfg(feature = "serde")]
mod serde {
    use super::FileChecksum;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl<const HASH_LEN: usize> Serialize for FileChecksum<HASH_LEN> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, const HASH_LEN: usize> Deserialize<'de> for FileChecksum<HASH_LEN> {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

#[cfg(feature = "hex")]
mod hex {
    #![cfg_attr(docsrs, doc(cfg(feature = "hex")))]

    use super::FileChecksum;

    impl<const HASH_LEN: usize> FileChecksum<HASH_LEN> {
        /// Return the parsed digest for this File.
        pub fn digest(&self) -> [u8; HASH_LEN] {
            self.digest.digest()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::FileChecksum;
        use crate::control::changes::HASH_LEN_SHA256;
        use ::hex;

        #[test]
        fn hex_digest_sha256() {
            let file = FileChecksum::<HASH_LEN_SHA256> {
                digest: "e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba"
                    .parse()
                    .unwrap(),
                size: 1183,
                path: "hello_2.10-3.dsc".to_owned(),
            };

            assert_eq!(
                hex::decode("e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba")
                    .unwrap(),
                file.digest.digest(),
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{HASH_LEN_SHA1, HASH_LEN_SHA256};
    use super::*;

    #[test]
    fn test_parse_sha1() {
        let file: FileChecksum<HASH_LEN_SHA1> =
            "4755bb94240986213836726f9b594e853920f541 1183 hello_2.10-3.dsc"
                .parse()
                .unwrap();

        assert_eq!(1183, file.size);
        assert_eq!("hello_2.10-3.dsc", file.path);
    }

    #[test]
    fn test_parse_sha256() {
        let file: FileChecksum<HASH_LEN_SHA256> =
             "e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba 1183 hello_2.10-3.dsc"
                .parse()
                .unwrap();

        assert_eq!(1183, file.size);
        assert_eq!("hello_2.10-3.dsc", file.path);
    }
}

// vim: foldmethod=marker
