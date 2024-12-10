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

//! The `arch` module contains support for parsing Debian architecture
//! strings.
//!
//! There's two major flavors for Debian. The first (the [Architecture] enum)
//! is the one that we all know and love (the one you see when you pick an
//! install medium, or install a package), and the second is the
//! [multiarch::Tuple], seen in Debian multiarch paths, or with cross-builds.
//! They are similar to, but different from GNU Triplets.
//!
//! Every effort is made to correctly handle the known Architectures and Tuples,
//! but this module will fall back to treat it as valid but unknown.
//!
//! ```
//! use deb::architecture::Architecture;
//!
//! // Get the multiarch tuple for the `amd64` arch, this prints
//! // "x86_64-linux-gnu", which is of type [deb::arch::multiarch::Tuple].
//! println!("{}", Architecture::Amd64.multiarch_tuple().unwrap());
//! ```
//!
//! # Feature `serde`
//!
//! This feature will enable derives or explicit implementations of
//! [serde::Deserialize] and [serde::Serialize] for types in this module.

#[allow(clippy::module_inception)]
mod architecture;
pub mod multiarch;

pub use architecture::{Architecture, Error};

// vim: foldmethod=marker
