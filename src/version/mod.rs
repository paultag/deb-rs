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

//! The `version` module implements support for comparing Debian
//! package [Version]s.
//!
//! Debian has a long history of using a very specific version syntax, which
//! has very well defined comparison rules, which are used to manage the
//! versioning and comparison of package versions throughout the project.
//!
//! These versions are *not* the same as "semver", although "semver" versions
//! will compare correctly as a Debian [Version].
//!
//! ```
//! use deb::version::Version;
//!
//! let v: Version = "1:1.0-1".parse().unwrap();
//!
//! // prints "1"
//! println!("{}", v.epoch().unwrap());
//!
//! // prints "1.0"
//! println!("{}", v.upstream_version());
//!
//! // prints "1"
//! println!("{}", v.debian_revision().unwrap());
//! ```
//!
//! # Feature `serde`
//!
//! This feature will enable derives or explicit implementations of
//! [serde::Deserialize] and [serde::Serialize] for types in this module.

mod compare;
mod tests_dpkg;
#[allow(clippy::module_inception)]
mod version;

pub use version::{Error, Version};

// vim: foldmethod=marker
