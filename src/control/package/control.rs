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

use crate::control::{Number, Priority};
use crate::{architecture::Architecture, dependency::Dependency, version::Version};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Debian binary (`.deb`) binary control file (sometimes called
/// `DEBIAN/control` -- note the upper case), as seen in binary deb files.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct BinaryControl {
    /// Binary package name
    pub package: String,

    /// The value of this field determines the package name, and is used to
    /// generate file names by most installation tools.
    pub source: Option<String>,

    /// Typically, this is the original package's [Version] number in whatever
    /// form the program's author uses. It may also include a Debian revision
    /// number (for non-native packages).
    pub version: Version,

    /// Archive Section that this package belongs to.
    pub section: Option<String>,

    /// Priority of the binary package.
    pub priority: Option<Priority>,

    /// Lists the [crate::architecture::Architecture] of the files contained
    /// in this package. Common architectures are `amd64`, `armel`, `i386`,
    /// ([crate::architecture::AMD64],
    /// [crate::architecture::ARMEL],
    /// [crate::architecture::I386]), etc.
    pub architecture: Option<Architecture>,

    /// If set, and set to "`yes`", this package is an essential package,
    /// which has special-cased handling in `dpkg` and `apt`.
    pub essential: Option<String>,

    /// Size of the package's contents on-disk.
    #[cfg_attr(feature = "serde", serde(rename = "Installed-Size"))]
    pub installed_size: Option<Number<usize>>,

    /// Name and email of the package's maintainer.
    pub maintainer: String,

    /// Description of this binary package's purpose.
    pub description: String,

    /// The upstream project home page url.
    pub homepage: Option<String>,

    /// Packages that this binary package requires be installed in order to
    /// be fully installed.
    pub depends: Option<Dependency>,

    /// Packages which this binary package needs to be installed in all but
    /// the most unusual installs. Removing one may cause breakage if their
    /// purpose is not understood.
    pub recommends: Option<Dependency>,

    /// Packages which this binary package must not be installed at the same
    /// time as.
    pub conflicts: Option<Dependency>,

    /// Packages which could be interesting to be installed along with this
    /// package.
    pub suggests: Option<Dependency>,

    /// Packages that were used to produce this binary file.
    ///
    /// This is used from within the archive to ensure that source packages
    /// are not removed when their source is still included in a binary,
    /// but it may also be helpful to use when tracking down issues or
    /// triaging what packages need to be rebuilt.
    #[cfg_attr(feature = "serde", serde(rename = "Built-Using"))]
    pub built_using: Option<Dependency>,

    /// Packages which will become broken by the installation of this binary
    /// package.
    pub breaks: Option<Dependency>,

    /// Package makes another package better.
    pub enhances: Option<Dependency>,

    /// Packages which must be installed before this binary begins to
    /// unpack.
    pub pre_depends: Option<Dependency>,
}

// #[cfg(test)]
// mod tests {
//     #[cfg(feature = "serde")]
//     use super::*;
//
//     #[cfg(feature = "serde")]
//     mod serde {
//         use super::*;
//         use crate::{architecture, control::de};
//
//         macro_rules! test_package {
//             ($name:ident, $data:expr, |$parsed:ident| $block:tt) => {
//                 #[test]
//                 fn $name() {
//                     let $parsed = de::from_str::<Package>($data).unwrap();
//                     $block
//                 }
//             };
//         }
// }

// vim: foldmethod=marker
