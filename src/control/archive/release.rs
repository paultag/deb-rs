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

use crate::control::{
    Architectures, CommaDelimitedStrings, DateTime2822, FileDigestMd5, FileDigestSha1,
    FileDigestSha256, FileDigestSha512, SpaceDelimitedStrings,
};

/// Debian archive `Release` file, as seen at filepaths like
/// `dists/*/InRelease` on repositories designed for use with
/// `apt`.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Release {
    /// Description regarding this archive's release. Something like
    /// "Debian 12.8 Released 09 November 2024"
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub description: Option<String>,

    /// Entity that produced this Archive. Set to "Debian" for the Debian
    /// archives, but can really be anything descriptive.
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub origin: Option<String>,

    /// Archive label. Typically used extensively in repositories split over
    /// multiple media such as repositories stored on CDs.
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub label: Option<String>,

    /// Version of this Release.
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub version: Option<String>,

    /// Archive Suite name, something like "unstable" or what have you.
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub suite: Option<String>,

    /// Codename of the release. The suite name could be something like "stable"
    /// and over time, the codename may change.
    ///
    /// This field may be displayed to the user by package management tool or
    /// used for pinning. It is suggested that any repository published for
    /// other users to use fills meaningful information in these fields so that
    /// the user can tell apart different repositories.
    pub codename: Option<String>,

    /// Archive components which are installable. These are referenced in the
    /// apt `sources.list.d` file(s).
    pub components: Option<SpaceDelimitedStrings>,

    /// Available release architecture(s) that this repository has available,
    /// including the special arch `all`.
    pub architectures: Option<Architectures>,

    /// Date when the [Release] file was last updated.
    pub date: Option<DateTime2822>,

    /// Date after which this [Release] file must be considered "expired",
    /// requiring a newer version of the `Release` file. This helps to prevent
    /// downgrade attacks.
    pub valid_until: Option<DateTime2822>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// Note: The MD5 checksum is considered weak, and should never be assumed
    /// to be sufficient for secure verification, but this field cannot be
    /// omitted as it provides metadata not available anywhere else.
    #[cfg_attr(feature = "serde", serde(rename = "MD5Sum"))]
    pub md5sums: Option<Vec<FileDigestMd5>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// Note: The SHA-1 checksum is considered weak, and should never be
    /// assumed to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "SHA1"))]
    pub sha1: Option<Vec<FileDigestSha1>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    #[cfg_attr(feature = "serde", serde(rename = "SHA256"))]
    pub sha256: Option<Vec<FileDigestSha256>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    #[cfg_attr(feature = "serde", serde(rename = "SHA512"))]
    pub sha512: Option<Vec<FileDigestSha512>>,

    /// If a value of "yes" is specified for the NotAutomatic field, a package
    /// manager should not install packages (or upgrade to newer versions)
    /// from this repository without explicit user consent (APT assigns
    /// priority 1 to this)
    pub not_automatic: Option<bool>,

    /// If the field ButAutomaticUpgrades is specified as well and has the value
    /// "yes", the package manager should automatically install package upgrades
    /// from this repository, if the installed version of the package is higher
    /// than the version of the package in other sources (APT assigns priority
    /// 100).
    ///
    /// Specifying "yes" for ButAutomaticUpgrades without specifying "yes" for
    /// NotAutomatic is invalid.
    pub but_automatic_upgrades: Option<bool>,

    /// An optional boolean field with the default value "no". A value of "yes"
    /// indicates that the server supports the optional "by-hash" locations as
    /// an alternative to the canonical location (and name) of an index file.
    /// A client is free to choose which locations it will try to get indexes
    /// from, but it is recommend to use the "by-hash" location if supported
    /// by the server for its benefits for servers and clients. A client may
    /// fallback to the canonical location if by-hash fails.
    #[cfg_attr(feature = "serde", serde(rename = "Acquire-By-Hash"))]
    pub acquire_by_hash: Option<bool>,

    /// An optional field containing a comma separated list of OpenPGP key
    /// fingerprints to be used for validating the next Release file. The
    /// fingerprints must consist only of hex digits and may not contain
    /// spaces. The fingerprint specifies either the key the Release file must
    /// be signed with or the key the signature key must be a subkey of. The
    /// later match can be disabled by appending an exclamation mark to the
    /// fingerprint.
    ///
    /// If the field is present, a client should only accept future updates to
    /// the repository that are signed with keys listed in the field. The field
    /// should be ignored if the Valid-Until field is not present or if it is
    /// expired.
    #[cfg_attr(feature = "serde", serde(rename = "Signed-By"))]
    pub signed_by: Option<CommaDelimitedStrings>,

    /// A hint that package downloads will require authorization. This allows
    /// clients to prevent use of that repository if authorization has not been
    /// provided, avoiding problems with failing downloads.
    ///
    /// Deprecated. Will be removed in a future apt version, as it breaks local
    /// mirroring of such repos if the local mirror is unauthorized but apt
    /// requires auth data and the local http server rejects authorization
    /// credentials for unprotected resources.
    #[cfg_attr(feature = "serde", serde(rename = "Packages-Require-Authorization"))]
    pub packages_require_authorization: Option<bool>,

    /// The Changelogs field tells the client where to find changelogs.
    pub changelogs: Option<String>,

    /// The Snapshots field tells the client where to find snapshots for this
    /// archive. The variable @SNAPSHOTID@ is substituted for the snapshot ID,
    /// which should be a timestamp in the form of the `%Y%m%dT%H%M%SZ` time
    /// format string.
    pub snapshots: Option<String>,
}

// vim: foldmethod=marker
