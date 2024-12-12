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
//! There's two major flavors for Debian. The first (the [Architecture] struct)
//! is the one that we all know and love (the one you see when you pick an
//! install medium, or install a package), and the second is the
//! [multiarch::Tuple], seen in Debian multiarch paths, or with cross-builds.
//! They are similar to, but different from GNU Triplets.
//!
//! ```
//! use deb::architecture::Architecture;
//!
//! // Print the architecture name for the Debian `amd64` distro, which
//! // outputs, unsuprisingly, `amd64`.
//! println!("{}", Architecture::AMD64);
//!
//! // Get the multiarch tuple for the `amd64` arch, this prints
//! // "x86_64-linux-gnu", which is of type [deb::arch::multiarch::Tuple].
//! println!("{}", Architecture::AMD64.multiarch_tuple().unwrap());
//! ```
//!
//! ## Note ♫
//!
//! Confusingly, both [Architecture] and [multiarch::Tuple] valus may grow
//! dashes (`-`) in such a way that they may become easy to confuse.
//! For instance, `x86_64-linux-gnu` is a [multiarch::Tuple], and
//! `base-gnu-linux-amd64` is an [Architecture] -- but both refer to the
//! same system configuration.
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
