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

use super::{PackageList, HASH_LEN_MD5, HASH_LEN_SHA1, HASH_LEN_SHA256};
use crate::{
    control::{changes::FileChecksum, Architectures, CommaDelimitedStrings},
    dependency::Dependency,
    version::Version,
};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Error conditions which may be encountered when working with a [Dsc]
/// file.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DscParseError {
    /// This is, unfortunately, a very generic and generally unhelpful
    /// error. This is returned if *something* wasn't as expected with the
    /// file.
    ///
    /// There are a few cases where a [DscParseError::Malformed] is returned
    /// where something else would have been more helpful. A lot of places
    /// that return a `Malformed` today are likely to change error cases
    /// in the future.
    Malformed,
}
crate::errors::error_enum!(DscParseError);

/// When preparing a package upload for Debian, the source package's
/// manifest is generated in the form of a `.dsc`.
///
/// These fields are clearsigned with a specific Developer or buildd
/// machine's OpenPGP key.
///
/// There's a lot more information `dsc(5)`, and some of the
/// fields in this struct contain text written in that manpage.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dsc {
    /// The value of this field declares the format version of the source
    /// package.  The field value is used by programs acting on a source
    /// package to interpret the list of files in the source package and
    /// determine how to unpack it. The syntax of the field value is a
    /// numeric major revision (`0-9`), a period (`.`), a numeric minor
    /// revision (`0-9`), and then an optional subtype after whitespace
    /// (` \t`), which if specified is a lowercase alphanumeric (`a-z0-9`)
    /// word in parentheses (`()`). The subtype is optional in the syntax but
    /// may be mandatory for particular source format revisions.
    ///
    /// The source formats currently supported by `dpkg` are `1.0`, `2.0`,
    /// `3.0 (native)`, `3.0 (quilt)`, `3.0 (git)`, `3.0 (bzr)` and
    /// `3.0 (custom)`. See dpkg-source(1) for their description.
    #[cfg_attr(feature = "serde", serde(rename = "Format"))]
    pub format: String,

    /// The value of this field determines the package name, and is used to
    /// generate file names by most installation tools.
    #[cfg_attr(feature = "serde", serde(rename = "Source"))]
    pub source: String,

    /// This folded field lists binary packages which this source package can
    /// produce, separated by commas.
    ///
    /// This field has now been superseded by the `Package-List` field, which
    /// gives enough information about what binary packages are produced on
    /// which architecture, build-profile and other involved restrictions.
    #[cfg_attr(feature = "serde", serde(rename = "Binary"))]
    pub binary: Option<CommaDelimitedStrings>,

    /// Lists the [crate::architecture::Architecture] of the files currently
    /// being uploaded. Common architectures are `amd64`, `armel`, `i386`,
    /// ([crate::architecture::Architecture::Amd64],
    /// [crate::architecture::Architecture::Armel],
    /// [crate::architecture::Architecture::I386]), etc. Note that the all value
    /// is meant for packages that are architecture independent. If the source
    /// for the package is also being uploaded, the special entry source is also
    /// present. Architecture wildcards must never be present in the list.
    #[cfg_attr(feature = "serde", serde(rename = "Architecture"))]
    pub architecture: Option<Architectures>,

    /// Typically, this is the original package's [Version] number in whatever
    /// form the program's author uses. It may also include a Debian revision
    /// number (for non-native packages).
    #[cfg_attr(feature = "serde", serde(rename = "Version"))]
    pub version: Version,

    /// Should be in the format `Joe Bloggs <jbloggs@foo.com>`, and is
    /// typically the person who created the package, as opposed to the
    /// author of the software that was packaged.
    #[cfg_attr(feature = "serde", serde(rename = "Maintainer"))]
    pub maintainer: String,

    /// Lists all the names and email addresses of co-maintainers of the
    /// package, in the same format as the Maintainer field. Multiple
    /// co-maintainers should be separated by a comma.
    #[cfg_attr(feature = "serde", serde(rename = "Uploaders"))]
    pub uploaders: Option<CommaDelimitedStrings>,

    /// The format for the source package description is a short brief summary
    /// on the first line (after the Description field). The following lines
    /// should be used as a longer, more detailed description. Each line of
    /// the long description must be preceded by a space, and blank lines in
    /// the long description must contain a single `.` following the preceding
    /// space.
    #[cfg_attr(feature = "serde", serde(rename = "Description"))]
    pub description: Option<String>,

    /// The upstream project home page url.
    #[cfg_attr(feature = "serde", serde(rename = "Homepage"))]
    pub homepage: Option<String>,

    /// Version Control information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub vcs: Option<Vcs>,

    /// Website to view this package in Version Control.
    #[cfg_attr(feature = "serde", serde(rename = "Vcs-Browser"))]
    browser: Option<String>,

    /// This field declares that the source package contains the specified test
    /// suites. The value is a comma-separated list of test suites. If the
    /// autopkgtest value is present, a `debian/tests/control` is expected to
    /// be present, if the file is present but not the value, then
    /// `dpkg-source` will automatically add it, preserving previous values.
    #[cfg_attr(feature = "serde", serde(rename = "Testsuite"))]
    pub testsuite: Option<CommaDelimitedStrings>,

    /// This field declares the comma-separated union of all test dependencies
    /// (`Depends` fields in `debian/tests/control` file), with all restrictions
    /// removed, and OR dependencies flattened (that is, converted to separate
    /// AND relationships), except for binaries generated by this source
    /// package and its meta-dependency equivalent @.
    ///
    /// Rationale: this field is needed because otherwise to be able to get the
    /// test dependencies, each source package would need to be unpacked.
    #[cfg_attr(feature = "serde", serde(rename = "Testsuite-Triggers"))]
    pub testsuite_triggers: Option<CommaDelimitedStrings>,

    /// Folded field containing a single git commit hash, presented in full,
    /// followed optionally by whitespace and other data to be defined in
    /// future extensions.
    #[cfg_attr(feature = "serde", serde(rename = "Dgit"))]
    pub dgit: Option<String>,

    /// This documents the most recent version of the distribution policy
    /// standards this package complies with.
    #[cfg_attr(feature = "serde", serde(rename = "Standards-Version"))]
    pub standards_version: String,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Depends"))]
    pub build_depends: Option<Dependency>,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Depends-Indep"))]
    pub build_depends_indep: Option<Dependency>,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Depends-Arch"))]
    pub build_depends_arch: Option<Dependency>,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Conflicts"))]
    pub build_conflicts: Option<Dependency>,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Conflicts-Indep"))]
    pub build_conflicts_indep: Option<Dependency>,

    /// This field declares relationships between the source package and
    /// packages used to build it. They are discussed in the
    /// `deb-src-control(5)` manual page.
    #[cfg_attr(feature = "serde", serde(rename = "Build-Conflicts-Arch"))]
    pub build_conflicts_arch: Option<Dependency>,

    /// list of binary packages generated by this source package.
    #[cfg_attr(feature = "serde", serde(rename = "Package-List"))]
    pub package_list: Vec<PackageList>,

    /// Files contains a list of files with an md5sum, size, section and
    /// priority for each one.
    ///
    /// Each line consists of space-separated entries describing the
    /// file: the md5sum, the file size, the file section, the file priority,
    /// and the file name.
    ///
    /// This field lists all files that make up the upload. The list of files
    /// in this field must match the list of files in the other related
    /// Checksums fields.
    ///
    /// Note: The MD5 checksum is considered weak, and should never be assumed
    /// to be sufficient for secure verification, but this field cannot be
    /// omitted as it provides metadata not available anywhere else.
    #[cfg_attr(feature = "serde", serde(rename = "Files"))]
    pub files: Vec<FileChecksum<HASH_LEN_MD5>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Checksums fields.
    ///
    /// Note: The SHA-1 checksum is considered weak, and should never be
    /// assumed to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Sha1"))]
    pub checksum_sha1: Option<Vec<FileChecksum<HASH_LEN_SHA1>>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Checksums fields.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Sha256"))]
    pub checksum_sha256: Vec<FileChecksum<HASH_LEN_SHA256>>,
}

/// Information regarding where the package's version control can be
/// obtained from.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Vcs {
    /// Git Version Control.
    Git {
        /// URL to git clone. The syntax of this field should
        /// be `<url> [ " -b " <branch> ] [ " [" <path> "]" ]`.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Git"))]
        url: String,
    },

    /// Subversion Version Control
    Svn {
        /// URL to svn clone.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Svn"))]
        url: String,
    },

    /// Arch Version Control.
    Arch {
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Arch"))]
        url: String,
    },

    /// Bazaar Version Control.
    Bzr {
        /// URL to fetch source from.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Bzr"))]
        url: String,
    },

    /// CVS Version Control.
    Cvs {
        /// URL to fetch source from.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Cvs"))]
        url: String,
    },

    /// Darcs Version Control.
    Darcs {
        /// URL to fetch source from.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Darcs"))]
        url: String,
    },

    /// Mercurial Version Control.
    Hg {
        /// URL to fetch source from.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Hg"))]
        url: String,
    },

    /// Monotone Version Control.
    Mtn {
        /// URL to fetch source from.
        #[cfg_attr(feature = "serde", serde(rename = "Vcs-Mtn"))]
        url: String,
    },
}

// vim: foldmethod=marker
