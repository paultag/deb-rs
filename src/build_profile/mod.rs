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

//! The `build_profile` module contains support for parsing Debian build profile
//! strings.
//!
//! A [BuildProfile] is used to assist with breaking complex package
//! relationships, such as is the case with bootstrapping the Debian
//! distribution, or cross-building.
//!
//! You can learn more about Build Profiles on the
//! Debian [Wiki](https://wiki.debian.org/BuildProfileSpec).
//!
//! ```
//! use deb::build_profile::BuildProfile;
//!
//! let bp: BuildProfile = "noudeb".parse().unwrap();
//!
//! assert_eq!(BuildProfile::NoUdeb, bp);
//!
//! // Prints "noudeb", unsuprisingly.
//! println!("{}", bp);
//! ```

#[allow(clippy::module_inception)]
mod build_profile;

pub use build_profile::{BuildProfile, Error};

// vim: foldmethod=marker
