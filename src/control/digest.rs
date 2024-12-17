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

#[cfg(not(feature = "hex"))]
type InnerDateTime<const HASH_LEN: usize> = String;

#[cfg(feature = "hex")]
type InnerDateTime<const HASH_LEN: usize> = [u8; HASH_LEN];

const HASH_LEN_MD5: usize = 16;
const HASH_LEN_SHA1: usize = 20;
const HASH_LEN_SHA256: usize = 32;
const HASH_LEN_SHA512: usize = 64;

/// [Digest] is a specific File's hash digest.
#[derive(Clone, Debug, PartialEq)]
pub struct Digest<const HASH_LEN: usize>(InnerDateTime<HASH_LEN>);

/// [Digest] for the MD5 digest algorithm.
///
/// This entry contains a now very antiquated `md5` digest, which should
/// _not_ be used for basically any purpose.
pub type DigestMd5 = Digest<HASH_LEN_MD5>;

/// [Digest] for the SHA1 digest algorithm.
///
/// This entry contains the now mostly broken `sha1` digest, which should
/// _not_ be used for any cryptographic purpose.
pub type DigestSha1 = Digest<HASH_LEN_SHA1>;

/// [Digest] for the SHA256 digest algorithm.
pub type DigestSha256 = Digest<HASH_LEN_SHA256>;

/// [Digest] for the SHA512 digest algorithm.
pub type DigestSha512 = Digest<HASH_LEN_SHA512>;

/// Error conditions which may be encountered when working with a [Digest]
/// field.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DigestParseError {
    /// Digest was empty! Can't turn that into a [Digest].
    Empty,

    /// Digest was of the wrong length. The [Digest] parsing code expects
    /// that the input data is ASCII encoded Hex. The ASCII value should be
    /// exactly twice the length of the raw hash length in bytes.
    BadLength,

    /// The Digest contained invalid Hex. This is only returned under
    /// the `hex` feature.
    InvalidEncoding,
}
crate::errors::error_enum!(DigestParseError);

#[cfg(not(feature = "hex"))]
mod no_hex {
    use super::*;

    impl<const HASH_LEN: usize> std::fmt::Display for Digest<HASH_LEN> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }

    impl<const HASH_LEN: usize> FromStr for Digest<HASH_LEN> {
        type Err = DigestParseError;
        fn from_str(checksum: &str) -> Result<Self, DigestParseError> {
            if checksum.is_empty() {
                return Err(DigestParseError::Empty);
            }
            if checksum.len() != (HASH_LEN * 2) {
                return Err(DigestParseError::BadLength);
            }
            Ok(Self(checksum.to_owned()))
        }
    }
}

#[cfg(feature = "hex")]
mod hex {
    use super::*;
    use ::hex;

    impl<const HASH_LEN: usize> std::fmt::Display for Digest<HASH_LEN> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", hex::encode(self.0))
        }
    }

    impl<const HASH_LEN: usize> FromStr for Digest<HASH_LEN> {
        type Err = DigestParseError;
        fn from_str(checksum: &str) -> Result<Self, DigestParseError> {
            if checksum.is_empty() {
                return Err(DigestParseError::Empty);
            }
            let checksum = hex::decode(checksum).map_err(|_| DigestParseError::InvalidEncoding)?;
            Ok(Self(
                checksum
                    .try_into()
                    .map_err(|_| DigestParseError::BadLength)?,
            ))
        }
    }

    impl<const HASH_LEN: usize> Digest<HASH_LEN> {
        /// Return the raw digest as bytes.
        #[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
        pub fn digest(&self) -> [u8; HASH_LEN] {
            self.0
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Digest;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    impl<const HASH_LEN: usize> Serialize for Digest<HASH_LEN> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, const HASH_LEN: usize> Deserialize<'de> for Digest<HASH_LEN> {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse::<Digest<HASH_LEN>>()
                .map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_parses {
        ($name:ident, $type:ty: $checksum:expr ) => {
            #[test]
            fn $name() {
                assert!($checksum.parse::<$type>().is_ok());
            }
        };
    }

    macro_rules! check_fails {
        ($name:ident, $type:ty: $checksum:expr ) => {
            #[test]
            fn $name() {
                assert!($checksum.parse::<$type>().is_err());
            }
        };
    }

    check_fails!(empty_md5, DigestMd5: "");
    check_fails!(empty_sh1, DigestSha1: "");
    check_fails!(empty_sha256, DigestSha256: "");
    check_fails!(empty_sha512, DigestSha512: "");

    check_parses!(good_md5, DigestMd5: "d41d8cd98f00b204e9800998ecf8427e");
    check_parses!(good_sha1, DigestSha1: "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    check_parses!(good_sha256, DigestSha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    check_parses!(good_sha512, DigestSha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");

    check_fails!(bad_md5, DigestMd5: "d41d8cd98f00b204e9800998ecf8427");
    check_fails!(bad_sha1, DigestSha1: "da39a3ee5e6b4b0d3255bfef95601890afd8070");
    check_fails!(bad_sha256, DigestSha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b85");
    check_fails!(bad_sha512, DigestSha512: "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3");

    #[cfg(feature = "hex")]
    mod hex {
        use super::*;

        check_fails!(malformed_md5, DigestMd5: "d41d8cd98f00bHACK9800998ecf8427e");
        check_fails!(malformed_sha1, DigestSha1: "da39a3ee5e6b4b0d3HACKfef95601890afd80709");
        check_fails!(malformed_sha256, DigestSha256: "e3b0c44298fc1c149afbf4HACK6fb92427ae41e4649b934ca495991b7852b855");
        check_fails!(malformed_sha512, DigestSha512: "cf83e1357eefb8bHACK02850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }
}

// vim: foldmethod=marker
