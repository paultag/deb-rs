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
//! what it will encode or decode, and the only first-class targets at
//! the moment are the files this crate itself parses. Over time, as this
//! crate matures, this warning will be removed.
//!
//! ## Restrictions / Rules on `serde` support
//!
//! There are three types of Debian RFC2822-style key/value types. Those
//! types are `simple` (the field must be on all one line), `folded` (multiple
//! lines, but no semantic meaning), and `multiline` (repeated continuation
//! lines of a specific format following a possible value on the same line
//! as the key).
//!
//! This crate treats `simple` and `folded` fields as the same. This may
//! break with a strict intepreation of control files, but it is only more
//! lax in what it produces. This may change in the future, do not rely
//! on this behavior.
//!
//! `multiline` fields will be treated as a `simple` or `folded` field if
//! they're unpacked into most fields, with the exception of unpacking a
//! `multiline` into a `Vec<_>`, which will parse each line into the provided
//! `Deserialize` traited target. *`multiline` files with a leading value on
//! the same line as the key are not supported as of right now*. If you're
//! parsing such a file, your best bet is to implement [serde::Deserialize]
//! yourself until this crate matures.
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
mod date_time;
mod delimited_strings;
mod paragraph;
mod pest;
mod real_control_tests;
mod traits;

pub mod changes;
pub mod dsc;

#[cfg(feature = "serde")]
pub mod de;

#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "sequoia")]
mod openpgp;

pub use architectures::Architectures;
pub use date_time::{DateTime2822, DateTime2822ParseError};
pub use delimited_strings::{CommaDelimitedStrings, SpaceDelimitedStrings};
pub use paragraph::{Error, RawField, RawParagraph};
pub use traits::FileEntry;

#[cfg(feature = "sequoia")]
pub use openpgp::{OpenPgpValidator, OpenPgpValidatorBuilder, OpenPgpValidatorError};

// vim: foldmethod=marker
