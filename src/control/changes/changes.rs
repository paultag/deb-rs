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

use super::{Closes, File, FileChecksum, Files, HASH_LEN_SHA1, HASH_LEN_SHA256};
use crate::{
    control::{Architectures, DateTime2822, SpaceDelimitedStrings},
    version::Version,
};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Error conditions which may be encountered when working with a [Changes]
/// file.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChangesParseError {
    /// This is, unfortunately, a very generic and generally unhelpful
    /// error. This is returned if *something* wasn't as expected with the
    /// file.
    ///
    /// There are a few cases where a [ChangesParseError::Malformed] is returned
    /// where something else would have been more helpful. A lot of places
    /// that return a `Malformed` today are likely to change error cases
    /// in the future.
    Malformed,

    /// A hash contained in the Changes file was of an invalid length for
    /// the algorithm type expected in that field.
    InvalidHashLength,

    /// A hash contained in the Changes file contained invalid values, or
    /// is otherwise a bad ASCII represntation of the digest.
    InvalidHash,

    /// A date wasn't able to be parsed from text or was otherwise
    /// invalid.
    InvalidDate,
}

/// When preparing a package upload for Debian, the upload action is done
/// by sending a `.changes` file to the Debian infrastructure, which
/// contains source, binaries and/or other files such as metadata about
/// the build.
///
/// These fields are clearsigned with a specific Developer or buildd
/// machine's OpenPGP key.
///
/// There's a lot more information `deb-changes(5)`, and some of the
/// fields in this struct contain text written in that manpage.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Changes {
    /// The value of this field declares the format version of the file. The
    /// syntax of the field value is a version number with a major and minor
    /// component. Backward incompatible changes to the format will bump the
    /// major version, and backward compatible changes (such as field
    /// additions) will bump the minor version.  The current format version
    /// is `1.8`.
    #[cfg_attr(feature = "serde", serde(rename = "Format"))]
    pub format: String,

    /// The date the package was built or last edited. It must be in the same
    /// format as the date in a deb-changelog(5) entry. The value of this
    /// field is usually extracted from the debian/changelog file.
    #[cfg_attr(feature = "serde", serde(rename = "Date"))]
    pub date: DateTime2822,

    /// The name of the source package in the format of
    /// `source-name [(source-version)]`. If the source version differs from
    /// the binary version, then the `source-name` will be followed by a
    /// `source-version` in parenthesis. This can happen when the upload is
    /// a binary-only non-maintainer upload.
    #[cfg_attr(feature = "serde", serde(rename = "Source"))]
    pub source: String,

    /// This folded field is a space-separated list of binary packages to
    /// upload. If the upload is source-only, then the field is omitted
    /// (since dpkg 1.19.3).
    #[cfg_attr(feature = "serde", serde(rename = "Binary"))]
    pub binary: Option<SpaceDelimitedStrings>,

    /// Lists the [crate::architecture::Architecture] of the files currently
    /// being uploaded. Common architectures are `amd64`, `armel`, `i386`,
    /// ([crate::architecture::Architecture::Amd64],
    /// [crate::architecture::Architecture::Armel],
    /// [crate::architecture::Architecture::I386]), etc. Note that the all value
    /// is meant for packages that are architecture independent. If the source
    /// for the package is also being uploaded, the special entry source is also
    /// present. Architecture wildcards must never be present in the list.
    #[cfg_attr(feature = "serde", serde(rename = "Architecture"))]
    pub architecture: Architectures,

    /// Typically, this is the original package's [Version] number in whatever
    /// form the program's author uses. It may also include a Debian revision
    /// number (for non-native packages).
    #[cfg_attr(feature = "serde", serde(rename = "Version"))]
    pub version: Version,

    /// Lists one or more space-separated distributions where this version
    /// should be installed when it is uploaded to the archive.
    #[cfg_attr(feature = "serde", serde(rename = "Distribution"))]
    pub distribution: String,

    /// The urgency of the upload. The currently known values, in increasing
    /// order of urgency, are: low, medium, high, critical and emergency.
    #[cfg_attr(feature = "serde", serde(rename = "Urgency"))]
    pub urgency: String,

    /// Should be in the format "Joe Bloggs <jbloggs@example.org>", and is
    /// typically the person who created the package, as opposed to the
    /// author of the software that was packaged.
    #[cfg_attr(feature = "serde", serde(rename = "Maintainer"))]
    pub maintainer: String,

    /// Should be in the format "Joe Bloggs <jbloggs@example.org>", and is
    /// typically the person who prepared the package changes for this release.
    #[cfg_attr(feature = "serde", serde(rename = "Changed-By"))]
    pub changed_by: Option<String>,

    /// This multiline field contains a list of binary package names followed
    /// by a space, a dash ('-') and their possibly truncated short descriptions.
    /// If the upload is source-only, then the field is omitted.
    #[cfg_attr(feature = "serde", serde(rename = "Description"))]
    pub description: Option<String>,

    /// A space-separated list of bug report numbers for bug reports that have
    /// been resolved with this upload.  The distribution archive software might
    /// use this field to automatically close the referred bug numbers in the
    /// distribution bug tracking system.
    #[cfg_attr(feature = "serde", serde(rename = "Closes"))]
    pub closes: Option<Closes>,

    /// This field denotes that the upload is a binary-only non-maintainer
    /// build. It originates from the binary-only=yes key/value from the
    /// changelog metadata entry.
    #[cfg_attr(feature = "serde", serde(rename = "Binary-Only"))]
    pub binary_only: Option<bool>,

    /// This field specifies a whitespace separated list of build profiles that
    /// this upload was built with.
    #[cfg_attr(feature = "serde", serde(rename = "Built-For-Profiles"))]
    pub built_for_profiles: Option<SpaceDelimitedStrings>,

    /// This multiline field contains the concatenated text of all changelog
    /// entries that are part of the upload. The exact content depends on the
    /// changelog format.
    #[cfg_attr(feature = "serde", serde(rename = "Changes"))]
    pub changes: String,

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
    pub files: Files<File>,

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
    pub checksum_sha1: Option<Files<FileChecksum<HASH_LEN_SHA1>>>,

    /// Each line consists of space-separated entries describing the file:
    /// the checksum, the file size, and the file name.
    ///
    /// These fields list all files that make up the upload. The list of files
    /// in these fields must match the list of files in the Files field and
    /// the other related Checksums fields.
    #[cfg_attr(feature = "serde", serde(rename = "Checksums-Sha256"))]
    pub checksum_sha256: Files<FileChecksum<HASH_LEN_SHA256>>,
}

#[cfg(feature = "serde")]
mod serde {
    #[cfg(test)]
    mod tests {
        use crate::{
            architecture::Architecture,
            control::{
                self,
                changes::{Changes, File, Files},
            },
        };
        use std::io::{BufReader, Cursor};

        #[test]
        fn test_changes() {
            let mut reader = BufReader::new(Cursor::new("\
Format: 1.8
Date: Mon, 26 Dec 2022 16:30:00 +0100
Source: hello
Binary: hello hello-dbgsym
Architecture: source amd64
Version: 2.10-3
Distribution: unstable
Urgency: medium
Maintainer: Santiago Vila <sanvila@debian.org>
Changed-By: Santiago Vila <sanvila@debian.org>
Description:
 hello      - example package based on GNU hello
Closes: 871622 893083
Changes:
 hello (2.10-3) unstable; urgency=medium
 .
   * Add some autopkgtests. Closes: #871622.
   * Add Vcs-Git and Vcs-Browser fields to debian/control. Closes: #893083.
   * Raise debhelper compat level from 9 to 13. This enables autoreconf,
     and as a result, some additional build-dependencies are required:
   - Add texinfo to Build-Depends, for a normal build.
   - Add help2man to Build-Depends, for a build using git.
   * Use secure URI in Homepage field.
   * Set upstream metadata fields Bug-Submit, Name and Repository-Browse.
   * Add upstream signing-key.
   * Use a common debian/watch file which is valid for most GNU packages.
   * Sort control fields using wrap-and-sort.
   * Update standards version to 4.6.2.
Checksums-Sha1:
 4755bb94240986213836726f9b594e853920f541 1183 hello_2.10-3.dsc
 82e477ec77f09bae910e53592d28319774754af6 12688 hello_2.10-3.debian.tar.xz
 45a6ecadd0d8672ab875451c17f84067137783c8 36084 hello-dbgsym_2.10-3_amd64.deb
 9a6e6d94a7bbf07e8d8f46071dbaa3fc9c0f1227 7657 hello_2.10-3_amd64.buildinfo
 8439082041b2b154fdb48f98530cbdf54557abac 53324 hello_2.10-3_amd64.deb
Checksums-Sha256:
 e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba 1183 hello_2.10-3.dsc
 f43ddcca8d7168c5d52b53e1f2a69b78f42f8387633ef8955edd0621c73cf65c 12688 hello_2.10-3.debian.tar.xz
 16990db381cd1816fc65436447dedaa3298fc29179ee7e4379e7793a7d75cacb 36084 hello-dbgsym_2.10-3_amd64.deb
 ae955f1835dd9948fa6b8aaeb6f26aff21ff6501a41913ae52306aa2d627f918 7657 hello_2.10-3_amd64.buildinfo
 052cb5fdfa86bb3485d6194d9ae2fd1cabbccbdd9c7da3258aed1674b288bbf9 53324 hello_2.10-3_amd64.deb
Files:
 e7bd195571b19d33bd83d1c379fe6432 1183 devel optional hello_2.10-3.dsc
 16678389ba7fddcdfa05e0707d61f043 12688 devel optional hello_2.10-3.debian.tar.xz
 5b2bcd51a3ad0d0e611aafd9276b938e 36084 debug optional hello-dbgsym_2.10-3_amd64.deb
 57144f2c9158564350da3371b5b9a542 7657 devel optional hello_2.10-3_amd64.buildinfo
 d36abefbc87d8dfb7704238f0aee0e90 53324 devel optional hello_2.10-3_amd64.deb
"));

            let changes: Changes = control::de::from_reader(&mut reader).unwrap();

            assert_eq!("hello", changes.source);
            assert_eq!(2, changes.binary.unwrap().len());
            assert_eq!(2, changes.architecture.len());
            assert_eq!(
                &[Architecture::Source, Architecture::Amd64,],
                changes.architecture.as_ref(),
            );

            assert_eq!(2, changes.closes.as_ref().unwrap().as_ref().len());
            assert_eq!(
                &["871622", "893083"],
                changes.closes.as_ref().unwrap().as_ref()
            );

            assert_eq!(5, changes.files.len());
            assert_eq!(
                Files(vec![
                    File {
                        digest: "e7bd195571b19d33bd83d1c379fe6432".to_owned(),
                        size: 1183,
                        path: "hello_2.10-3.dsc".to_owned(),
                        section: "devel".to_owned(),
                        priority: "optional".to_owned(),
                    },
                    File {
                        digest: "16678389ba7fddcdfa05e0707d61f043".to_owned(),
                        size: 12688,
                        path: "hello_2.10-3.debian.tar.xz".to_owned(),
                        section: "devel".to_owned(),
                        priority: "optional".to_owned()
                    },
                    File {
                        digest: "5b2bcd51a3ad0d0e611aafd9276b938e".to_owned(),
                        size: 36084,
                        path: "hello-dbgsym_2.10-3_amd64.deb".to_owned(),
                        section: "debug".to_owned(),
                        priority: "optional".to_owned()
                    },
                    File {
                        digest: "57144f2c9158564350da3371b5b9a542".to_owned(),
                        size: 7657,
                        path: "hello_2.10-3_amd64.buildinfo".to_owned(),
                        section: "devel".to_owned(),
                        priority: "optional".to_owned()
                    },
                    File {
                        digest: "d36abefbc87d8dfb7704238f0aee0e90".to_owned(),
                        size: 53324,
                        path: "hello_2.10-3_amd64.deb".to_owned(),
                        section: "devel".to_owned(),
                        priority: "optional".to_owned()
                    }
                ]),
                changes.files,
            );

            assert_eq!(changes.files.len(), changes.checksum_sha1.unwrap().len());
            assert_eq!(changes.files.len(), changes.checksum_sha256.len());
        }
    }
}

// vim: foldmethod=marker
