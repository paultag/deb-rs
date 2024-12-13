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

use crate::{architecture::Architecture, dependency::Dependency, version::Version};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Debian archive Package index file, as seen in
/// `dists/unstable/main/binary-amd64/Packages.xz` and friends.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Package {
    /// Binary package name
    #[cfg_attr(feature = "serde", serde(rename = "Package"))]
    pub package: String,

    /// The value of this field determines the package name, and is used to
    /// generate file names by most installation tools.
    #[cfg_attr(feature = "serde", serde(rename = "Source"))]
    pub source: Option<String>,

    /// Typically, this is the original package's [Version] number in whatever
    /// form the program's author uses. It may also include a Debian revision
    /// number (for non-native packages).
    #[cfg_attr(feature = "serde", serde(rename = "Version"))]
    pub version: Version,

    /// Archive Section that this package belongs to.
    #[cfg_attr(feature = "serde", serde(rename = "Section"))]
    pub section: String,

    /// Priority of the binary package.
    #[cfg_attr(feature = "serde", serde(rename = "Priority"))]
    pub priority: String,

    /// Lists the [crate::architecture::Architecture] of the files contained
    /// in this package. Common architectures are `amd64`, `armel`, `i386`,
    /// ([crate::architecture::Architecture::AMD64],
    /// [crate::architecture::Architecture::ARMEL],
    /// [crate::architecture::Architecture::I386]), etc.
    #[cfg_attr(feature = "serde", serde(rename = "Architecture"))]
    pub architecture: Option<Architecture>,

    /// If set, and set to "`yes`", this package is an essential package,
    /// which has special-cased handling in `dpkg` and `apt`.
    #[cfg_attr(feature = "serde", serde(rename = "Essential"))]
    pub essential: Option<String>,

    /// Size of the package's contents on-disk.
    #[cfg_attr(feature = "serde", serde(rename = "Installed-Size"))]
    pub installed_size: Option<usize>,

    /// Name and email of the package's maintainer.
    #[cfg_attr(feature = "serde", serde(rename = "Maintainer"))]
    pub maintainer: String,

    /// Description of this binary package's purpose.
    #[cfg_attr(feature = "serde", serde(rename = "Description"))]
    pub description: String,

    /// The upstream project home page url.
    #[cfg_attr(feature = "serde", serde(rename = "Homepage"))]
    pub homepage: Option<String>,

    /// Path within the Debian archive to the specific `.deb` file.
    #[cfg_attr(feature = "serde", serde(rename = "Filename"))]
    pub filename: String,

    /// Size of the binary `.deb` file.
    #[cfg_attr(feature = "serde", serde(rename = "Size"))]
    pub size: usize,

    /// MD5 hash of the `.deb` file.
    ///
    /// Note: The MD5 checksum is considered weak, and should never be assumed
    /// to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "MD5sum"))]
    pub md5sum: String,

    /// SHA256 hash of the `.deb` file.
    #[cfg_attr(feature = "serde", serde(rename = "SHA256"))]
    pub sha256: String,

    /// MD5 hash of the package's full Description. The [Self::description]
    /// field only contains the short description.
    #[cfg_attr(feature = "serde", serde(rename = "Description-md5"))]
    pub description_md5: String,

    /// Packages that this binary package requires be installed in order to
    /// be fully installed.
    #[cfg_attr(feature = "serde", serde(rename = "Depends"))]
    pub depends: Option<Dependency>,

    /// Packages which this binary package needs to be installed in all but
    /// the most unusual installs. Removing one may cause breakage if their
    /// purpose is not understood.
    #[cfg_attr(feature = "serde", serde(rename = "Recommends"))]
    pub recommends: Option<Dependency>,

    /// Packages which this binary package must not be installed at the same
    /// time as.
    #[cfg_attr(feature = "serde", serde(rename = "Conflicts"))]
    pub conflicts: Option<Dependency>,

    /// Packages which could be interesting to be installed along with this
    /// package.
    #[cfg_attr(feature = "serde", serde(rename = "Suggests"))]
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
    #[cfg_attr(feature = "serde", serde(rename = "Breaks"))]
    pub breaks: Option<Dependency>,

    /// Packages which must be installed before this binary begins to
    /// unpack.
    #[cfg_attr(feature = "serde", serde(rename = "Pre-Depends"))]
    pub pre_depends: Option<Dependency>,
}

// vim: foldmethod=marker
