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

use crate::control::{Digest, DigestParseError};
use std::str::FromStr;

const HASH_LEN_MD5: usize = 16;
const HASH_LEN_SHA1: usize = 20;
const HASH_LEN_SHA256: usize = 32;
const HASH_LEN_SHA512: usize = 64;

/// [FileDigest] is a specific File's hash digest and filesize referenced
/// by a control file.
#[derive(Clone, Debug, PartialEq)]
pub struct FileDigest<const HASH_LEN: usize> {
    /// Hash digest of a File contained in this upload. The specific length
    /// of the digest is dicated by the hash algorithm.
    pub digest: Digest<HASH_LEN>,

    /// File size, in bytes, of the File contained in this upload.
    pub size: usize,

    /// Path of the file relative to the control file, usually a buildinfo
    /// or changes.
    pub path: String,
}

/// A [FileDigest] set to the `HASH_LEN` of MD5, 16 bytes.
///
/// MD5 is a broken hashing algorithm and shouldn't be used for cryptographic
/// purposes.
pub type FileDigestMd5 = FileDigest<HASH_LEN_MD5>;

/// A [FileDigest] set to the `HASH_LEN` of SHA1, 20 bytes.
///
/// SHA1 is a broken hashing algorithm and shouldn't be used for cryptographic
/// purposes.
pub type FileDigestSha1 = FileDigest<HASH_LEN_SHA1>;

/// A [FileDigest] set to the `HASH_LEN` of SHA256, 32 bytes.
pub type FileDigestSha256 = FileDigest<HASH_LEN_SHA256>;

/// A [FileDigest] set to the `HASH_LEN` of SHA512, 64 bytes.
pub type FileDigestSha512 = FileDigest<HASH_LEN_SHA512>;

impl<const HASH_LEN: usize> std::fmt::Display for FileDigest<HASH_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.digest, self.size, self.path,)
    }
}

/// Error conditions which may be encountered when working with a
/// [FileDigest].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileDigestParseError {
    /// Underlying checksum error when parsing the digest into a [Digest].
    DigestParseError(DigestParseError),

    /// The [FileDigest] line must take the format of `digest size path`,
    /// space delimited. If the line is malformed or differently structured,
    /// this will be returned.
    MalformedFileDigestLine,

    /// The file size couldn't be converted into a number.
    MalformedFileSize,
}
crate::errors::error_enum!(FileDigestParseError);

impl From<DigestParseError> for FileDigestParseError {
    fn from(cpe: DigestParseError) -> Self {
        Self::DigestParseError(cpe)
    }
}

impl<const HASH_LEN: usize> FromStr for FileDigest<HASH_LEN> {
    type Err = FileDigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [digest, size, path] = s
            .split(" ")
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| FileDigestParseError::MalformedFileDigestLine)?;

        Ok(Self {
            digest: digest.parse()?,
            size: size
                .parse()
                .map_err(|_| FileDigestParseError::MalformedFileSize)?,
            path: path.to_owned(),
        })
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::FileDigest;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

    impl<const HASH_LEN: usize> Serialize for FileDigest<HASH_LEN> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, const HASH_LEN: usize> Deserialize<'de> for FileDigest<HASH_LEN> {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{e:?}")))
        }
    }
}

#[cfg(feature = "hex")]
mod hex {
    #![cfg_attr(docsrs, doc(cfg(feature = "hex")))]

    use super::FileDigest;

    impl<const HASH_LEN: usize> FileDigest<HASH_LEN> {
        /// Return the parsed digest for this File.
        pub fn digest(&self) -> [u8; HASH_LEN] {
            self.digest.digest()
        }
    }

    #[cfg(test)]
    mod tests {
        const HASH_LEN_SHA256: usize = 32;
        use super::FileDigest;
        use ::hex;

        #[test]
        fn hex_digest_sha256() {
            let file = FileDigest::<HASH_LEN_SHA256> {
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
    const HASH_LEN_SHA1: usize = 20;
    const HASH_LEN_SHA256: usize = 32;
    use super::*;

    #[test]
    fn test_parse_sha1() {
        let file: FileDigest<HASH_LEN_SHA1> =
            "4755bb94240986213836726f9b594e853920f541 1183 hello_2.10-3.dsc"
                .parse()
                .unwrap();

        assert_eq!(1183, file.size);
        assert_eq!("hello_2.10-3.dsc", file.path);
    }

    #[test]
    fn test_parse_sha256() {
        let file: FileDigest<HASH_LEN_SHA256> =
             "e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba 1183 hello_2.10-3.dsc"
                .parse()
                .unwrap();

        assert_eq!(1183, file.size);
        assert_eq!("hello_2.10-3.dsc", file.path);
    }
}

// vim: foldmethod=marker
