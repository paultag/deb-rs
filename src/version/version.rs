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

//! The `version` module implements support for comparing Versions.

// This is a from-docs reimplementation of the algorithm described in
// `deb-version(5)`. I did not use the `dpkg` source writing this,
// and the test cases I pulled are only used when building in test mode,
// in the `tests_dpkg.rs` file. I've seen those copied around a lot, but
// if there's an issue with those version comparisons being somehow
// copyrightable, i'll write some new test cases. I have a bunch of my own
// throughout as well.

use std::str::FromStr;

/// Debian package version number.
///
/// These versions are *not* the same as "semver", although "semver" versions
/// will compare correctly as a Debian [Version].
///
/// a Debian [Version] takes the form of
/// `[epoch:]upstream-version[-debian-revision]`.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Version {
    /// This is a single (generally small) unsigned integer. It may be omitted.
    /// If it is omitted then the upstream-version may not contain any colons.
    ///
    /// `dpkg` will refuse to parse `epoch` values larger than `i32`, but we're
    /// using a `u64` to avoid storage issues. It shouldn't change, because
    /// `epoch` should be rarely used, and kept very small.
    epoch: Option<u64>,

    /// This is the main part of the version number. It is usually the version
    /// number of the original ("upstream") package from which the package has
    /// been made, if applicable. Usually this will be in the same format
    /// as that specified by the upstream author(s); however, it may need to be
    /// reformatted to fit into the comparison scheme.
    ///
    /// The `upstream_version` portion of the version number is mandatory.
    /// The upstream-version may contain only alphanumerics (`A-Za-z0-9`) and
    /// the characters `.` `+` `-` `:` `~` (full stop, plus, hyphen, colon,
    /// tilde) and should start with a digit. If there is no `debian_revision`
    /// then hyphens are not allowed; if there is no epoch then colons are not
    /// allowed.
    upstream_version: String,

    /// This part of the version number specifies the version of the Debian
    /// package based on the upstream version.
    ///
    /// It may contain only alphanumerics and the characters `+` `.` `~`
    /// (plus, full stop, tilde) and is compared in the same way as the
    /// `upstream_version` is. It is optional. If it isn't present then the
    /// `upstream_version` may not contain a hyphen. This format represents the
    /// case where a piece of software was written specifically to be turned
    /// into a Debian package, and so there is only one "debianization" of it
    /// and therefore no revision indication is required. It is conventional to
    /// restart the `debian_revision` at `1` each time the `upstream_version`
    /// is increased. Dpkg will break the version number apart at the last
    /// hyphen in the string (if there is one) to determine the
    /// `upstream_version` and `debian_revision`. The absence of a
    /// `debian_revision` compares earlier than the presence of one (but note
    /// that the debian-revision is the least significant part of the version
    /// number).
    debian_revision: Option<String>,
}

/// Error conditions which may be encountered when parsing a String
/// into a [Version].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// Returned when the string provided to [Version] is empty.
    Empty,

    /// Something was so broken there's no more specific error about this.
    /// This Error state will be fixed over time, and hopefully removed.
    Malformed,

    /// The Version had an invalid epoch. An epoch must be an unsigned integer
    /// less than [i32::MAX].
    InvalidEpoch,

    /// The `upstream_version` component of the [Version] is empty.
    NoUpstreamVersion,

    /// The `debian_revision` component of the [Version] is empty, but
    /// it can't be based on the [Version] rules around inclusion of `-`
    /// in the `upstream_version`.
    NoDebianRevision,

    /// The `upstream_version` contains chars which are not permitted.
    InvalidUpstreamVersion,

    /// The `debian_revision` contains chars which are not permitted.
    InvalidDebianRevision,
}

impl Version {
    /// Create a new Version, and verify that the constructed Version is
    /// valid from parts.
    pub fn from_parts(
        epoch: Option<u64>,
        upstream_version: &str,
        debian_revision: Option<&str>,
    ) -> Result<Self, Error> {
        let ret = Version {
            epoch,
            upstream_version: upstream_version.to_owned(),
            debian_revision: debian_revision.map(|v| v.to_owned()),
        };
        ret.check()?;
        Ok(ret)
    }

    /// Return the `epoch` of the [Version].
    pub fn epoch(&self) -> Option<u64> {
        self.epoch
    }

    /// Return the `upstream_version` of the [Version]. This must be
    /// compared according to the dpkg [Version] comparison rules.
    pub fn upstream_version(&self) -> &str {
        &self.upstream_version
    }

    /// Return the `debian_revision` of the [Version]. This must be
    /// compared according to the dpkg [Version] comparison rules.
    pub fn debian_revision(&self) -> Option<&str> {
        self.debian_revision.as_deref()
    }

    /// Check that the version is permissible.
    fn check(&self) -> Result<(), Error> {
        if let Some(ch) = self.upstream_version.chars().next() {
            if !ch.is_ascii_digit() {
                return Err(Error::InvalidUpstreamVersion);
            }
        }
        for ch in self.upstream_version.chars() {
            if ch.is_ascii_lowercase()
                || ch.is_ascii_uppercase()
                || ch.is_ascii_digit()
                || ch == '~'
                || ch == '+'
                || ch == '.'
            {
                continue;
            }
            if ch == ':' && self.epoch.is_some() {
                // ":" is only allowed if the Epoch os set.
                continue;
            }
            if ch == '-' && self.debian_revision.is_some() {
                // "-" is only allowed if the Debian version is set
                continue;
            }
            return Err(Error::InvalidUpstreamVersion);
        }

        if let Some(debian_revision) = &self.debian_revision {
            for ch in debian_revision.chars() {
                if ch.is_ascii_lowercase()
                    || ch.is_ascii_uppercase()
                    || ch.is_ascii_digit()
                    || ch == '~'
                    || ch == '+'
                    || ch == '.'
                {
                    continue;
                }
                return Err(Error::InvalidDebianRevision);
            }
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(mut ver: &str) -> Result<Self, Error> {
        ver = ver.trim();
        let mut ret: Self = Default::default();

        match ver.splitn(2, ':').collect::<Vec<_>>()[..] {
            [version] => {
                ver = version;
            }
            [epoch, version] => {
                let epoch = epoch.parse().map_err(|_| Error::InvalidEpoch)?;
                // i32 INT_MAX is a dpkg constraint.
                if epoch > (i32::MAX as u64) {
                    return Err(Error::InvalidEpoch);
                }
                ret.epoch = Some(epoch);
                ver = version;
            }
            _ => {
                return Err(Error::Malformed);
            }
        }

        if ver.is_empty() {
            return Err(Error::Empty);
        }

        match ver.rsplitn(2, '-').collect::<Vec<_>>()[..] {
            [upstream_version] => {
                ret.upstream_version = upstream_version.to_owned();
            }
            [debian_revision, upstream_version] => {
                if debian_revision.is_empty() {
                    return Err(Error::NoDebianRevision);
                }

                if upstream_version.is_empty() {
                    return Err(Error::NoUpstreamVersion);
                }

                ret.upstream_version = upstream_version.to_owned();
                ret.debian_revision = Some(debian_revision.to_owned());
            }
            _ => {
                return Err(Error::Malformed);
            }
        }

        if ret.upstream_version.is_empty() {
            return Err(Error::NoUpstreamVersion);
        }

        ret.check()?;

        Ok(ret)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (&self.debian_revision, &self.epoch) {
                (Some(debian_revision), Some(epoch)) => {
                    format!("{}:{}-{}", epoch, self.upstream_version, debian_revision)
                }
                (None, Some(epoch)) => {
                    format!("{}:{}", epoch, self.upstream_version)
                }
                (Some(debian_revision), None) => {
                    format!("{}-{}", self.upstream_version, debian_revision)
                }
                (None, None) => self.upstream_version.clone(),
            }
        )
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Version;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::control;
        use std::io::{BufReader, Cursor};

        #[test]
        fn serde_version() {
            #[derive(Clone, Debug, PartialEq, Deserialize)]
            struct Test {
                #[serde(rename = "Version")]
                version: Version,
            }

            let test: Test = control::de::from_reader(&mut BufReader::new(Cursor::new(
                "\
Version: 1:1.0-1
",
            )))
            .unwrap();

            assert_eq!(test.version.epoch, Some(1));
            assert_eq!(test.version.upstream_version, "1.0");
            assert_eq!(test.version.debian_revision, Some("1".to_owned()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_matches {
        ($name:ident, $version:expr, $check:expr) => {
            #[test]
            fn $name() {
                let v: Version = $version.parse().unwrap();
                assert_eq!($check, v);
            }
        };
    }

    macro_rules! check_validation_fails {
        ($name:ident, $version:expr) => {
            #[test]
            fn $name() {
                assert!($version.check().is_err());
            }
        };
    }

    macro_rules! check_parse_fails {
        ($name:ident, $version:expr) => {
            #[test]
            fn $name() {
                assert!($version.parse::<Version>().is_err());
            }
        };
    }

    macro_rules! check_fuzz_regression {
        ($name:ident, $version:expr) => {
            #[test]
            fn $name() {
                let Ok(v) = $version.parse::<Version>() else {
                    return;
                };
                v.to_string();

                let v2: Version = "100:100.100+100-100onehundred100~100".parse().unwrap();

                let _ = v.cmp(&v2);
                let _ = v2.cmp(&v);
            }
        };
    }

    check_matches!(
        simple_version,
        "1.0-1",
        Version {
            upstream_version: "1.0".to_owned(),
            debian_revision: Some("1".to_owned()),
            ..Default::default()
        }
    );

    check_matches!(
        simple_version_epoch,
        "1:1.0-1",
        Version {
            epoch: Some(1),
            upstream_version: "1.0".to_owned(),
            debian_revision: Some("1".to_owned()),
        }
    );

    check_matches!(
        spaces,
        "   1.0-1  ",
        Version {
            upstream_version: "1.0".to_owned(),
            debian_revision: Some("1".to_owned()),
            ..Default::default()
        }
    );

    check_matches!(
        all_zeros,
        "0:0:0:0-0",
        Version {
            epoch: Some(0),
            upstream_version: "0:0:0".to_owned(),
            debian_revision: Some("0".to_owned()),
        }
    );

    check_matches!(
        all_the_things,
        "0:09azAZ.-+~:-0",
        Version {
            epoch: Some(0),
            upstream_version: "09azAZ.-+~:".to_owned(),
            debian_revision: Some("0".to_owned()),
        }
    );

    check_matches!(
        all_the_things_revision,
        "0:0-azAZ09.+~",
        Version {
            epoch: Some(0),
            upstream_version: "0".to_owned(),
            debian_revision: Some("azAZ09.+~".to_owned()),
        }
    );

    check_parse_fails!(empty, "");
    check_parse_fails!(empty_space, "  ");
    check_parse_fails!(invalid_epoch, "-1:1.0-1");
    check_parse_fails!(invalid_epoch2, "1.0:1-1");
    check_parse_fails!(invalid_epoch3, "1:");
    check_parse_fails!(invalid_epoch4, "a:1.0");
    check_parse_fails!(invalid_upstream, "-1");
    check_parse_fails!(starting_number, "abc3-0");
    check_parse_fails!(space_twixt, "0:0 0-1");
    check_parse_fails!(invalid_chars1, "1.0@");
    check_parse_fails!(invalid_chars2, "1.0#");
    check_parse_fails!(empty_revision, "7-");
    check_parse_fails!(epoch_too_large, "333333333333333333:3");

    check_validation_fails!(
        invalid_construction_col,
        Version {
            epoch: None,
            upstream_version: "1:0".to_string(),
            debian_revision: Some("1".to_string()),
        }
    );
    check_validation_fails!(
        invalid_construction_dash,
        Version {
            epoch: Some(1),
            upstream_version: "1-1".to_string(),
            debian_revision: None,
        }
    );

    #[test]
    fn version_sort() {
        let mut versions = vec![
            "1.3",
            "1.0",
            "1.0+dfsg1-1",
            "1.0-1",
            "1.1",
            "0:1.2",
            "1:0.1",
            "1.0+dfsg1",
            "1.0~dfsg1",
        ]
        .into_iter()
        .map(|v| v.parse::<Version>().unwrap())
        .collect::<Vec<_>>();

        versions.sort();

        assert_eq!(
            vec![
                "1.0~dfsg1",
                "1.0",
                "1.0-1",
                "1.0+dfsg1",
                "1.0+dfsg1-1",
                "1.1",
                "0:1.2",
                "1.3",
                "1:0.1",
            ]
            .into_iter()
            .map(|v| v.parse::<Version>().unwrap())
            .collect::<Vec<_>>(),
            versions
        );
    }

    check_fuzz_regression!(
        long_number,
        "100:222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222221~~~~~~~~~~~~~~~~~1~1~0"
    );
}

// vim: foldmethod=marker
