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

//! The `control` module contains support for parsing Debian RFC 2822-style
//! files into our conventional formats.
//!
//! # Feature `serde`
//!
//! ⚠️  Support for directly using [ser] and [de] to encode and decode
//! arbitrary Debian-flavored RFC2822 files is possible but not recommended
//! yet. The Serializer and Deserializer implementation is very strict on
//! what it will encode or decode, and I don't particularly want to make
//! it very clever. It may be worth checking out Jelmer's
//! [deb822](https://github.com/jelmer/deb822-rs) project for that.
//!
//! This will export two modules from this package - [ser] and [de].
//! Additionally the crate will add [serde::Serialize] and [serde::Deserialize]
//! derives as required.
//!
//! # Feature `sequoia`
//!
//! This will add *very* basic OpenPGP support in a non-exported and
//! private module, as well as one additional function which is exported
//! if `serde` support is enabled as well under [de::from_clearsigned_str].
//!
//! # Feature `tokio`
//!
//! This will add support for reading [crate::control] files from a
//! [tokio::io::AsyncRead], via [de::from_reader_async]

mod architectures;
pub mod changes;
mod date_time;
mod paragraph;
mod pest;
mod real_control_tests;
mod space_delimited_strings;
mod traits;

#[cfg(feature = "serde")]
pub mod de;

#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "sequoia")]
mod openpgp;

pub use architectures::Architectures;
pub use date_time::{DateTime2822, DateTime2822ParseError};
pub use paragraph::{Error, RawField, RawParagraph};
pub use space_delimited_strings::SpaceDelimitedStrings;
pub use traits::FileEntry;

#[cfg(feature = "sequoia")]
pub use openpgp::{OpenPgpValidator, OpenPgpValidatorBuilder, OpenPgpValidatorError};

// vim: foldmethod=marker
