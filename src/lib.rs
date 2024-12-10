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

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

//! 🎉 You found the `deb` crate! 🎉
//!
//! This crate is under active development, and "soft launched". *Please do
//! not post widely directing to this crate yet* -- the API shipped today is
//! unstable, and is likely to change -- fairly significantly -- without much
//! regard to very precisely following semver until it stabalizes.
//!
//! You're more than welcome to play with this and use it, but it's not
//! something I would encourage load bearing infrastructure to be written
//! with as of right now.
//!
//! # Introduction
//!
//! The `deb` crate contains utilities for working with files and formats
//! commonly found when working with Debian's project tooling, or
//! infrastructure.
//!
//! Common use-cases are broken out into modules in the `deb` crate namespace,
//! such as interacting with [control] files, parsing [dependency]
//! relationships between Debian packages, parsing and ordering [version]
//! numbers, or understanding Debian [architecture] strings.
//!
//! Docs can be found on [docs.rs](https://docs.rs/deb/latest/deb/),
//! and information about the latest release can be found on
//! [crates.io](https://crates.io/crates/deb).
//!
//! # Feature Flags
//!
//! There are a few feature flags. There's no standard way to document
//! the purpose and intent, so until that's a thing, here's a markdown
//! table.
//!
//! | Flag      | Description                                                              |
//! | --------- | ------------------------------------------------------------------------ |
//! | `full`    | Enable all optional features.                                            |
//! | `chrono`  | Enable parsing dates using the [chrono] crate.                           |
//! | `hex`     | Enable parsing ASCII hex values using the [hex] crate                    |
//! | `serde`   | Enable support for encoding and decoding using [serde]                   |
//! | `sequoia` | Enable support for validating OpenPGP signatures using [sequoia_openpgp] |
//! | `tokio`   | Enable support for the [tokio] crate.                                    |
//!
//! # Feature `chrono`
//!
//! Enable parsing dates from ASCII into a [chrono::DateTime].
//!
//! # Feature `hex`
//!
//! Enable parsing hashes from ASCII into bytes using the `hex` crate. This
//! is only really useful in places where you're validating things like
//! digests over files in [control::changes::Changes] files, or similar.
//!
//! # Feature `serde`
//!
//! This exports two new modules for working with control files, [control::de],
//! and [control::ser] to read or write (respectively) control files in the
//! Debian RFC2822-style format, as is our convention.
//!
//! # Feature `sequoia`
//!
//! Enable functions to verify Debian control files using the [sequoia_openpgp]
//! OpenPGP implementation. This will export a few helpers throughout
//! the crate, such as [control::de::from_clearsigned_str].
//!
//! # Feature `tokio`
//!
//! Enable functions to handle places where there's an i/o boundary that is
//! handled by [tokio::io] rather than [std::io].

pub mod architecture;
pub mod build_profile;
pub mod control;
pub mod dependency;
pub mod version;

// vim: foldmethod=marker
