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

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

use crate::{
    architecture::Architecture,
    control::{
        Architectures, DateTime2822, FileDigestMd5, FileDigestSha1, FileDigestSha256,
        SpaceDelimitedStrings,
    },
    dependency::Dependency,
    version::Version,
};

// TODO
//   - format enum
//   - validation of overly-lax fields, ensure that
//     the dependency lines are all flat relation:package, all packages
//     have an (=) version constraint, no arch constraints or build profile
//     constraints. There may be a package arch constraint though.

/// The Debian `.buildinfo` file contains information regarding the build
/// environment, including installed packages and configuration.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Buildinfo {
    /// The value of this field declares the format version of the file.
    /// The syntax of the field value is a version number with a major and
    /// minor component. Backward incompatible changes to the format will bump
    /// the major version, and backward compatible changes (such as field
    /// additions) will bump the minor version. The current format version is 1.0.
    pub format: String,

    /// This folded field is a space-separated list of binary packages built.
    /// If the build is source-only, then the field is omitted (since dpkg 1.20.0).
    pub binary: Option<SpaceDelimitedStrings>,

    /// The name of the source package.  If the source version differs from
    /// the binary version, then the source-name will be followed by a
    /// source-version in parenthesis. This can happen when the build is for a
    /// binary-only non-maintainer upload.
    pub source: String,

    /// This space-separated field lists the architectures of the files
    /// currently being built. Common architectures are amd64, armel, i386,
    /// etc. Note that the all value is meant for packages that are architecture
    /// independent. If the source for the package is also being built,
    /// the special entry source is also present. Architecture wildcards
    /// must never be present in the list.
    pub architecture: Architectures,

    /// Typically, this is the original package's version number in whatever
    /// form the program's author uses.  It may also include a Debian revision
    /// number (for non-native packages). The exact format and sorting
    /// algorithm are described in deb-version(7).
    pub version: Version,

    /// This multiline field contains the concatenated text of the changelog
    /// entry for a binary-only non-maintainer upload (binNMU) if that is the
    /// case.  To make this a valid multiline field empty lines are replaced
    /// with a single full stop (`.`) and all lines are indented by one space
    /// character. The exact content depends on the changelog format.
    #[cfg_attr(feature = "serde", serde(rename = "Binary-Only-Changes"))]
    pub binary_only_changes: Option<String>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Digests fields.
    ///
    /// Note: The MD5 checksum is considered weak, and should never be
    /// assumed to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Md5"))]
    pub checksum_md5: Option<Vec<FileDigestMd5>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Digests fields.
    ///
    /// Note: The SHA-1 checksum is considered weak, and should never be
    /// assumed to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Sha1"))]
    pub checksum_sha1: Option<Vec<FileDigestSha1>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Digests fields.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Sha256"))]
    pub checksum_sha256: Vec<FileDigestSha256>,

    /// The name of the distribution this package is originating from.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Origin"))]
    pub build_origin: Option<String>,

    /// The Debian architecture for the installation the packages is being
    /// built in. Common architectures are amd64, armel, i386, etc.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Architecture"))]
    pub build_architecture: Architecture,

    /// The date the package was built.  It must be in the same format as the
    /// date in a deb-changelog(5) entry.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Date"))]
    pub build_date: Option<DateTime2822>,

    /// The release and version (in an unspecified format) of the kernel
    /// running on the build system.  This field is only going to be present
    /// if the builder has explicitly requested it, to avoid leaking possibly
    /// sensitive information.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Kernel-Version"))]
    pub build_kernel_version: Option<String>,

    /// The absolute build path, which correspond to the unpacked source tree.
    /// This field is only going to be present if the vendor has allowed it
    /// via some pattern match to avoid leaking possibly sensitive information.
    ///
    /// On Debian and derivatives only build paths starting with `/build/`
    /// will emit this field.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Path"))]
    pub build_path: Option<String>,

    /// This folded field contains a space-separated list of non-exhaustive
    /// reason tags (formed by alphanumeric and dash characters) which identify
    /// why the current build has been tainted (since dpkg 1.19.5).
    #[cfg_attr(feature = "serde", serde(rename = "Build-Tained-By"))]
    pub build_tained_by: Option<SpaceDelimitedStrings>,

    /// The list of installed and configured packages that might affect the
    /// package build process.
    ///
    /// The list consists of each package name, optionally arch-qualified for
    /// foreign architectures, with an exact version restriction, separated
    /// by commas.
    ///
    /// The list includes all essential packages, packages listed in
    /// `Build-Depends`, `Build-Depends-Arch`, `Build-Depends-Indep` source
    /// control fields, any vendor specific builtin dependencies, and all their
    /// recursive dependencies.  On Debian and derivatives the dependency
    /// builtin is build-essential.
    ///
    /// For dependencies coming from the source control fields, all dependency
    /// alternatives and all providers of virtual packages depended on will be
    /// included.
    #[cfg_attr(feature = "serde", serde(rename = "Installed-Build-Depends"))]
    pub installed_build_depends: Dependency,

    /// The list of environment variables that are known to affect the package
    /// build process, with each environment variable followed by an equal sign
    /// (`=`) and the variable's quoted value, using double quotes (`"`),
    /// and backslashes escaped (`\\`).
    pub environment: Vec<String>,
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
