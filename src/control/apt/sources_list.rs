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

use super::YesNoForce;
use crate::control::{Architectures, SpaceDelimitedStrings};

// TODO: check which fields are optional; add tests

/// Information on where to fetch information regarding installable
/// Debian files, and optionally, their corresponding source.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct SourcesList {
    /// If not enabled, this source entry will be ignored.
    pub enabled: Option<bool>,

    /// If `deb` is present, this will be used to fetch `.deb` files from.
    /// If `deb-src` is present, this will also be used to fetch `.dsc` files
    /// and its manifested additional source files.
    pub types: SpaceDelimitedStrings,

    /// base of the Debian distribution, from which APT will find the
    /// information it needs. suite can specify an exact path, in which case
    /// the components must be omitted and suite must end with a slash (/).
    /// This is useful for the case when only a particular sub-directory of the
    /// archive denoted by the URI is of interest. If suite does not specify an
    /// exact path, at least one component must be present.
    #[cfg_attr(feature = "serde", serde(rename = "URIs"))]
    pub uris: SpaceDelimitedStrings,

    /// suite may also contain a variable, `$(ARCH)` which expands to the Debian
    /// architecture (such as amd64 or armel) used on the system. This permits
    /// architecture-independent sources.list files to be used. In general this
    /// is only of interest when specifying an exact path; APT will
    /// automatically generate a URI with the current architecture otherwise.
    pub suites: SpaceDelimitedStrings,

    /// Archive components to use (like `main`, `contrib`, `non-free`, etc).
    pub components: SpaceDelimitedStrings,

    /// Set which architectures information should be downloaded.
    pub architectures: Option<Architectures>,

    /// List ofe languages which language-specific information such as
    /// translated package descriptions should be downloaded.
    pub languages: Option<SpaceDelimitedStrings>,

    /// Targets apt will try to acquire from this source. If not specified,
    /// the default set is defined by the Acquire::IndexTargets configuration
    /// scope (targets are specified by their name in the Created-By field).
    pub targets: Option<SpaceDelimitedStrings>,

    /// Use PDiffs to update old indexes instead of downloading the new
    /// indexes entirely.
    #[cfg_attr(feature = "serde", serde(rename = "PDiffs"))]
    pub pdiffs: Option<bool>,

    /// can have the value yes, no or force and controls if APT should try to
    /// acquire indexes via a URI constructed from a hashsum of the expected
    /// file instead of using the well-known stable filename of the index.
    #[cfg_attr(feature = "serde", serde(rename = "By-Hash"))]
    pub by_hash: Option<YesNoForce>,

    /// Circumvent parts of `apt-secure(8)`. Don't do this casually.
    #[cfg_attr(feature = "serde", serde(rename = "Allow-Insecure"))]
    pub allow_insecure: Option<bool>,

    /// Circumvent parts of `apt-secure(8)`. Don't do this casually.
    #[cfg_attr(feature = "serde", serde(rename = "Allow-Weak"))]
    pub allow_weak: Option<bool>,

    /// Circumvent parts of `apt-secure(8)`. Don't do this casually.
    #[cfg_attr(feature = "serde", serde(rename = "Allow-Downgrade-To-Insecure"))]
    pub allow_downgrade_to_insecure: Option<bool>,

    /// Set the source to trusted or not, even if it doesn't pass authentication
    /// checks.
    ///
    ///
    /// This disables parts of `apt-secure(8)`, and should therefore only be
    /// used in a local and trusted context (if at all) as otherwise security
    /// is breached. The value no does the opposite, causing the source to be
    /// handled as untrusted even if the authentication checks passed
    /// successfully.
    pub trusted: Option<bool>,

    /// require a repository to pass apt-secure(8) verification with a certain
    /// set of keys rather than all trusted keys apt has configured.
    ///
    /// It is specified as a list of absolute paths to keyring files,
    /// and fingerprints of keys to select from these keyrings. If no
    /// fingerprint is specified all keys in the keyrings are selected. A
    /// fingerprint will accept also all signatures by a subkey of this key,
    /// if this isn't desired an exclamation mark (!) can be appended to the
    /// fingerprint to disable this behaviour.
    ///
    /// The option may also be set directly to an embedded GPG public key
    /// block.
    #[cfg_attr(feature = "serde", serde(rename = "Signed-By"))]
    pub signed_by: Option<String>,

    /// yes/no value which controls if APT should try to detect replay attacks.
    #[cfg_attr(feature = "serde", serde(rename = "Check-Valid-Until"))]
    pub check_valid_until: Option<bool>,

    /// lower the time period in seconds in which the data from this repository
    /// is considered valid.
    #[cfg_attr(feature = "serde", serde(rename = "Valid-Until-Min"))]
    pub valid_until_min: Option<usize>,

    /// raise the time period in seconds in which the data from this repository
    /// is considered valid.
    #[cfg_attr(feature = "serde", serde(rename = "Valid-Until-Max"))]
    pub valid_until_max: Option<usize>,

    /// consider the machine's time correct and hence perform time related
    /// checks, such as verifying that a Release file is not from the future.
    #[cfg_attr(feature = "serde", serde(rename = "Check-Date"))]
    pub check_date: Option<bool>,

    /// controls how far from the future a repository may be. Default to the
    /// value of the configuration option Acquire::Max-FutureTime which is 10
    /// seconds by default.
    #[cfg_attr(feature = "serde", serde(rename = "Date-Max-Future"))]
    pub date_max_future: Option<usize>,

    /// path to the InRelease file, relative to the normal position of an
    /// InRelease file.
    #[cfg_attr(feature = "serde", serde(rename = "InRelease-Path"))]
    pub inrelease_path: Option<String>,

    /// allows selecting an earlier version of the archive from the snapshot
    /// service.
    pub snapshot: Option<String>,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use super::*;

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;
        use crate::{architecture, control::de};

        macro_rules! test_sources_list {
            ($name:ident, $data:expr, |$parsed:ident| $block:tt) => {
                #[test]
                fn $name() {
                    let $parsed = de::from_str::<SourcesList>($data).unwrap();
                    $block
                }
            };
        }

        test_sources_list!(
            apt_manpage_example_1,
            "\
Types: deb
URIs: http://deb.debian.org/debian
Suites: bookworm bookworm-updates
Components: main contrib non-free non-free-firmware
",
            |sources| {
                assert_eq!(&["deb"], &*sources.types);
                assert_eq!(&["http://deb.debian.org/debian"], &*sources.uris);
                assert_eq!(&["bookworm", "bookworm-updates"], &*sources.suites);
                assert_eq!(
                    &["main", "contrib", "non-free", "non-free-firmware"],
                    &*sources.components
                );
            }
        );

        test_sources_list!(
            apt_manpage_signed_by,
            "\
Types: deb
URIs: https://deb.debian.org
Suites: stable
Components: main contrib non-free non-free-firmware
Signed-By:
 -----BEGIN PGP PUBLIC KEY BLOCK-----
 .
 mDMEYCQjIxYJKwYBBAHaRw8BAQdAD/P5Nvvnvk66SxBBHDbhRml9ORg1WV5CvzKY
 CuMfoIS0BmFiY2RlZoiQBBMWCgA4FiEErCIG1VhKWMWo2yfAREZd5NfO31cFAmAk
 IyMCGyMFCwkIBwMFFQoJCAsFFgIDAQACHgECF4AACgkQREZd5NfO31fbOwD6ArzS
 dM0Dkd5h2Ujy1b6KcAaVW9FOa5UNfJ9FFBtjLQEBAJ7UyWD3dZzhvlaAwunsk7DG
 3bHcln8DMpIJVXht78sL
 =IE0r
 -----END PGP PUBLIC KEY BLOCK-----
",
            |sources| {
                assert!(sources
                    .signed_by
                    .unwrap()
                    .trim()
                    .starts_with("-----BEGIN PGP PUBLIC KEY BLOCK-----"));
            }
        );

        test_sources_list!(
            apt_manpage_example_2,
            "\
Types: deb
URIs: http://deb.debian.org/debian
Suites: bookworm
Components: main
Architectures: amd64 armel
",
            |sources| {
                assert_eq!(
                    &[architecture::AMD64, architecture::ARMEL],
                    &*sources.architectures.unwrap()
                );
            }
        );

        test_sources_list!(
            apt_bool_one,
            "\
Types: deb
URIs: http://deb.debian.org/debian
Suites: bookworm
Components: main
Architectures: amd64 armel
Allow-Insecure: yes
",
            |sources| {
                assert!(sources.allow_insecure.unwrap());
            }
        );
    }
}

// vim: foldmethod=marker
