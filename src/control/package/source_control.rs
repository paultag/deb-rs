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

use crate::{
    control::{Architectures, CommaDelimitedStrings},
    dependency::Dependency,
    version::Version,
};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

// TODO:
//   - validation of optional fields that are contextually required

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
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct SourceControl {
    /// The value of this field determines the package name, and is used to
    /// generate file names by most installation tools.
    pub source: String,

    /// This folded field lists binary packages which this source package can
    /// produce, separated by commas.
    ///
    /// This field has now been superseded by the `Package-List` field, which
    /// gives enough information about what binary packages are produced on
    /// which architecture, build-profile and other involved restrictions.
    pub binary: Option<CommaDelimitedStrings>,

    /// Lists the [crate::architecture::Architecture] of the files currently
    /// being uploaded. Common architectures are `amd64`, `armel`, `i386`,
    /// ([crate::architecture::AMD64],
    /// [crate::architecture::ARMEL],
    /// [crate::architecture::I386]), etc. Note that the all value
    /// is meant for packages that are architecture independent. If the source
    /// for the package is also being uploaded, the special entry source is also
    /// present. Architecture wildcards must never be present in the list.
    pub architecture: Option<Architectures>,

    /// Typically, this is the original package's [Version] number in whatever
    /// form the program's author uses. It may also include a Debian revision
    /// number (for non-native packages).
    pub version: Version,

    /// Should be in the format `Joe Bloggs <jbloggs@foo.com>`, and is
    /// typically the person who created the package, as opposed to the
    /// author of the software that was packaged.
    pub maintainer: String,

    /// Lists all the names and email addresses of co-maintainers of the
    /// package, in the same format as the Maintainer field. Multiple
    /// co-maintainers should be separated by a comma.
    pub uploaders: Option<CommaDelimitedStrings>,

    /// The format for the source package description is a short brief summary
    /// on the first line (after the Description field). The following lines
    /// should be used as a longer, more detailed description. Each line of
    /// the long description must be preceded by a space, and blank lines in
    /// the long description must contain a single `.` following the preceding
    /// space.
    pub description: Option<String>,

    /// The upstream project home page url.
    pub homepage: Option<String>,

    /// Version Control information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub vcs: Option<Vcs>,

    /// Website to view this package in Version Control.
    #[cfg_attr(feature = "serde", serde(rename = "Vcs-Browser"))]
    pub browser: Option<String>,

    /// This field declares that the source package contains the specified test
    /// suites. The value is a comma-separated list of test suites. If the
    /// autopkgtest value is present, a `debian/tests/control` is expected to
    /// be present, if the file is present but not the value, then
    /// `dpkg-source` will automatically add it, preserving previous values.
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
    pub dgit: Option<String>,

    /// This documents the most recent version of the distribution policy
    /// standards this package complies with.
    ///
    /// This is optional if the package only produces udebs.
    #[cfg_attr(feature = "serde", serde(rename = "Standards-Version"))]
    pub standards_version: Option<String>,

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
