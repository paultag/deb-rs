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

//! The `release` module contains a best-effort basis copy of the Debian
//! release history and metadata to help with processing Debian files.
//!
//! Debian announces its new stable release on a regular basis.
//! The Debian release life cycle encompasses five years: the first three
//! years of full support followed by two years of Long Term Support (LTS).
//!
//! This module exports a [Release] type, which contains some interesting
//! metadata, as well as a bunch of `const` entries for releases. All Debian
//! releases at the time of module authoring are present in [RELEASES].
//!
//! # Release History
//!
//! | Name         | Version |
//! | ------------ | ------- |
//! | [DUKE]       | 15      |
//! | [FORKY]      | 14      |
//! | [TRIXIE]     | 13      |
//! | [BOOKWORM]   | 12      |
//! | [BULLSEYE]   | 11      |
//! | [BUSTER]     | 10      |
//! | [STRETCH]    | 9       |
//! | [JESSIE]     | 8       |
//! | [WHEEZY]     | 7.0     |
//! | [SQUEEZE]    | 6.0     |
//! | [LENNY]      | 5.0     |
//! | [ETCH]       | 4.0     |
//! | [SARGE]      | 3.1     |
//! | [WOODY]      | 3.0     |
//! | [POTATO]     | 2.2     |
//! | [SLINK]      | 2.1     |
//! | [HAMM]       | 2.0     |
//! | [BO]         | 1.3     |
//! | [REX]        | 1.2     |
//! | [BUZZ]       | 1.1     |
//!
//! # Feature `chrono`
//!
//! This feature will enable the loading of dates related to release
//! information via [Release::released_on] and [Release::eol_on].

use crate::architecture::{self, Architecture};
use std::borrow::Cow;

/// Metadata about a Debian stable release.
///
/// This contains information on when the release was promoted to
/// `stable` as well as when it fell out of official project support.
#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    /// Name of the Debian release - something like "`forky"` or
    /// "`sarge`".
    pub name: Cow<'static, str>,

    /// Debian release number. These are a bit all over the place and don't
    /// really encode any specific semantic meaning until [JESSIE] and later.
    pub version: Cow<'static, str>,

    /// Officially supported [Architecture] ports. Beyond official archive
    /// support, there's a handful of unoffical ports, which may have varying
    /// degrees of functionality and porters.
    pub architectures: Cow<'static, [Architecture]>,

    released_on: Option<NaiveDate>,
    eol_on: Option<NaiveDate>,
}

macro_rules! cow {
    ( $str:expr ) => {
        Cow::Borrowed($str)
    };
}

// Chrono support here

#[cfg(feature = "chrono")]
use ::chrono::NaiveDate;

#[cfg(feature = "chrono")]
macro_rules! date {
    ($y:literal/$m:literal/$d:literal) => {
        NaiveDate::from_ymd_opt($y, $m, $d)
    };
}

// *INTERNAL* facing stubs to mock out the chrono support. Nothing here
// should leave this file.

#[cfg(not(feature = "chrono"))]
type NaiveDate = ();

#[cfg(not(feature = "chrono"))]
macro_rules! date {
    ($_:expr) => {
        None
    };
}

/// Debian 1.1, Buzz
///
/// This was the first Debian release with a codename (everything before this
/// was just something like `0.9`. This release was fully `ELF` (not `a.out`
/// anymore), used Linux kernel 2.0.
pub const BUZZ: Release = Release {
    name: cow!("buzz"),
    version: cow!("1.1"),
    released_on: date!(1996 / 6 / 16),
    eol_on: date!(1996 / 12 / 12),
    architectures: cow!(&[architecture::I386]),
};

/// Debian 1.2, Rex
pub const REX: Release = Release {
    name: cow!("rex"),
    version: cow!("1.2"),
    released_on: date!(1996 / 12 / 12),
    eol_on: date!(1997 / 7 / 2),
    architectures: cow!(&[architecture::I386]),
};

/// Debian 1.3, Bo
///
/// This was the first release to support non-[architecture::I386]
/// architectures in the stable release -- and a lot of them, too! This
/// version saw the inclusion of [architecture::M68K], [architecture::ALPHA],
/// and [architecture::SPARC].
pub const BO: Release = Release {
    name: cow!("bo"),
    version: cow!("1.3"),
    released_on: date!(1997 / 7 / 2),
    eol_on: date!(1998 / 7 / 24),
    architectures: cow!(&[
        architecture::I386,
        architecture::M68K,
        architecture::ALPHA,
        architecture::SPARC
    ]),
};

/// Debian 2.0, Hamm
///
/// This release trimmed back on architecture support (a good thing, in
/// retrospect! Maintaining ports is hard!), only supporting
/// [architecture::I386] and [architecture::M68K]; with the rest of the
/// architectures from [BO] being only supported in `unstable`.
///
/// This also saw a huge `libc` transition (libc5 to libc6).
pub const HAMM: Release = Release {
    name: cow!("hamm"),
    version: cow!("2.0"),
    released_on: date!(1998 / 6 / 24),
    eol_on: date!(1999 / 3 / 9),

    // Alpha, Sparc, and PowerPC were in unstable.
    architectures: cow!(&[architecture::I386, architecture::M68K]),
};

/// Debian 2.1, Slink
///
/// `slink` saw the reintroduction of [architecture::ALPHA] and
/// [architecture::SPARC] as supported in the `stable` suite.
pub const SLINK: Release = Release {
    name: cow!("slink"),
    version: cow!("2.1"),
    released_on: date!(1999 / 3 / 9),
    eol_on: date!(2000 / 9 / 30),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::I386,
        architecture::M68K,
        architecture::SPARC,
    ]),
};

/// Debian 2.2, Potato
///
/// `potato` grew two new supported [Architecture]s, [architecture::ARM]
/// and [architecture::POWERPC].
///
/// This is also the first release featuring `apt`!
pub const POTATO: Release = Release {
    name: cow!("potato"),
    version: cow!("2.2"),
    released_on: date!(2000 / 8 / 15),
    eol_on: date!(2003 / 6 / 30),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::ARM,
        architecture::I386,
        architecture::M68K,
        architecture::POWERPC,
        architecture::SPARC,
    ]),
};

/// Debian 3.0, Woody
///
/// This was a big one! "Yee-haw! Giddy-up partner! We've got to get this wagon
/// train a-movin'!"
///
/// `woody` grew a bunch of new supported [Architecture]s, [architecture::HPPA],
/// [architecture::IA64], [architecture::MIPS], [architecture::MIPSEL] and
/// [architecture::S390].
pub const WOODY: Release = Release {
    name: cow!("woody"),
    version: cow!("3.0"),
    released_on: date!(2002 / 7 / 19),
    eol_on: date!(2006 / 6 / 30),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::ARM,
        architecture::HPPA,
        architecture::I386,
        architecture::IA64,
        architecture::M68K,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::SPARC,
    ]),
};

/// Debian 3.1, Sarge
pub const SARGE: Release = Release {
    // paultag's first Debian install!
    name: cow!("sarge"),
    version: cow!("3.1"),
    released_on: date!(2005 / 6 / 6),
    eol_on: date!(2008 / 3 / 31),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::ARM,
        architecture::HPPA,
        architecture::I386,
        architecture::IA64,
        architecture::M68K,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::SPARC,
    ]),
};

/// Debian 4.0, Etch
///
/// This was the first `stable` release with full official support for the
/// brand new [architecture::AMD64] [Architecture]! It was also the last
/// release with official [architecture::M68K] support, being dropped before
/// release.
pub const ETCH: Release = Release {
    name: cow!("etch"),
    version: cow!("4.0"),
    released_on: date!(2007 / 4 / 8),
    eol_on: date!(2012 / 2 / 6),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::AMD64,
        architecture::ARM,
        architecture::HPPA,
        architecture::I386,
        architecture::IA64,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::SPARC,
    ]),
};

/// Debian 5.0, Lenny
///
/// The new arm architecture [architecture::ARMEL] was officially supported
/// as of this release. This also deprecated the [architecture::ARM] support.
pub const LENNY: Release = Release {
    name: cow!("lenny"),
    version: cow!("5.0"),
    released_on: date!(2009 / 2 / 14),
    eol_on: date!(2012 / 2 / 6),
    architectures: cow!(&[
        architecture::ALPHA,
        architecture::AMD64,
        architecture::ARMEL,
        architecture::HPPA,
        architecture::I386,
        architecture::IA64,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::SPARC,
    ]),
};

/// Debian 6.0, Squeeze
///
/// The [architecture::HPPA] [Architecture] was dropped in this release.
///
/// This release grew two new technical previews --
/// [architecture::KFREEBSD_AMD64] and [architecture::KFREEBSD_I386] --
/// a port of the GNU userland, libc and Debian software ecosystem to run under
/// the FreeBSD kernel. These were not officially supported, however.
pub const SQUEEZE: Release = Release {
    name: cow!("squeeze"),
    version: cow!("6.0"),
    released_on: date!(2011 / 2 / 6),
    eol_on: date!(2014 / 5 / 31),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARMEL,
        architecture::I386,
        architecture::IA64,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::SPARC,
    ]),
};

/// Debian 7, Wheezy
///
/// Two new [Architecture]s here! `wheezy` was ported and offically supported
/// on [architecture::ARMHF] and [architecture::S390X].
pub const WHEEZY: Release = Release {
    // paultag's first Debian release as a DD
    name: cow!("wheezy"),
    version: cow!("7"),
    released_on: date!(2013 / 5 / 4),
    eol_on: date!(2016 / 4 / 25),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::IA64,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::S390,
        architecture::S390X,
        architecture::SPARC,
    ]),
};

/// Debian 8, Jessie
///
/// Yeee haw! [architecture::ARM64] and [architecture::PPC64EL] were added
/// as supported, and [architecture::S390], [architecture::SPARC] and
/// [architecture::IA64] were dropped.
pub const JESSIE: Release = Release {
    name: cow!("jessie"),
    version: cow!("8"),
    released_on: date!(2015 / 4 / 25),
    eol_on: date!(2018 / 6 / 17),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARM64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::MIPS,
        architecture::MIPSEL,
        architecture::POWERPC,
        architecture::PPC64EL,
        architecture::S390X,
    ]),
};

/// Debian 9, Stretch
///
/// [Release Announcement](https://lists.debian.org/debian-announce/2017/msg00003.html)
pub const STRETCH: Release = Release {
    name: cow!("stretch"),
    version: cow!("9"),
    released_on: date!(2017 / 6 / 17),
    eol_on: date!(2020 / 7 / 18),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARM64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::MIPS,
        architecture::MIPS64EL,
        architecture::MIPSEL,
        architecture::PPC64EL,
        architecture::S390X,
    ]),
};

/// Debian 10, Buster
///
/// [Release Announcement](https://lists.debian.org/debian-announce/2019/msg00003.html)
pub const BUSTER: Release = Release {
    name: cow!("buster"),
    version: cow!("10"),
    released_on: date!(2019 / 7 / 6),
    eol_on: date!(2022 / 9 / 10),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARM64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::MIPS,
        architecture::MIPS64EL,
        architecture::MIPSEL,
        architecture::PPC64EL,
        architecture::S390X,
    ]),
};

/// Debian 11, Bullseye
pub const BULLSEYE: Release = Release {
    name: cow!("bullseye"),
    version: cow!("11"),
    released_on: date!(2021 / 8 / 14),
    eol_on: date!(2024 / 8 / 14),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARM64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::MIPS64EL,
        architecture::MIPSEL,
        architecture::PPC64EL,
        architecture::S390X,
    ]),
};

/// Debian 12, Bookworm
///
/// [Release Announcement](https://www.debian.org/News/2023/20230610)
///
/// This was the first release after the Debian GR that introduced a new
/// archive area making it possible to separate non-free firmware from the
/// other non-free packages (`non-free-firmware` vs `non-free`).
pub const BOOKWORM: Release = Release {
    name: cow!("bookworm"),
    version: cow!("12"),
    released_on: date!(2023 / 6 / 10),
    eol_on: date!(2026 / 6 / 10),
    architectures: cow!(&[
        architecture::AMD64,
        architecture::ARM64,
        architecture::ARMEL,
        architecture::ARMHF,
        architecture::I386,
        architecture::MIPS64EL,
        architecture::MIPSEL,
        architecture::PPC64EL,
        architecture::S390X,
    ]),
};

// Hello, future person updating a release that just came out! Thank you
// for helping!
//
// If you wouldn't mind, please follow the following checklist:
//
//  - [ ] confirm the `released_on` date with the release team's announcement.
//  - [ ] confirm the `eol_on` date on the release team's announcement.
//  - [ ] confirm the `architectures` that newstable is releasing with.
//  - [ ] move this comment below the new stable release to just above
//        the next unreleased version.
//
// This action is likely enough to trigger a new crate release, so we should
// get that going after this!

/// Debian 13, Trixie
///
/// `trixie` is the first Debian release to support the
/// [architecture::RISCV64] [Architecture]. The [architecture::I386] port
/// is also in a bit of a limbo but not officially dropped, although it
/// no longer has an installer or kernel.
pub const TRIXIE: Release = Release {
    name: cow!("trixie"),
    version: cow!("13"),
    released_on: None,
    eol_on: None,

    // nothing is known until it releases. This one should have riscv64,
    // though!
    architectures: cow!(&[]),
};

/// Debian 14, Forky
pub const FORKY: Release = Release {
    name: cow!("forky"),
    version: cow!("14"),
    released_on: None,
    eol_on: None,

    // nothing is known until it releases.
    architectures: cow!(&[]),
};

/// Debian 15, Duke
pub const DUKE: Release = Release {
    name: cow!("duke"),
    version: cow!("15"),
    released_on: None,
    eol_on: None,

    // nothing is known until it releases.
    architectures: cow!(&[]),
};

// Hello, future person adding a new release! Thank you for helping!
//
// If you wouldn't mind, please add the new Release just above here,
// so that the releases go from oldest to newest. You can add them
// here when the release team announces the new stable codenames, and
// we can fix the `released_on` and `eol_on` date when we hit it.
//
//  - [ ] confirm the `name` with the release team's announcement.
//  - [ ] confirm the `version` with the release team's announcement.
//  - [ ] update the `RELEASE_HORIZON` to your best guess for just before
//        the next stable release.
//
// Add something that looks like this, but with real values:
//
// ```
// /// Debian 9999, Zurg
// pub const ZURG: Release = Release {
//     name: cow!("zurg"),
//     version: cow!("9999"),
//     released_on: None,
//     eol_on: None,
//     architectures: cow!(&[ ... ]),
// };
// ```
//
// This action is likely enough to trigger a new crate release, so we should
// get that going after this!

/// All Debian releases, historical, active and future, newest first.
pub const RELEASES: [Release; 19] = [
    DUKE, FORKY, TRIXIE, BOOKWORM, BULLSEYE, BUSTER, STRETCH, JESSIE, WHEEZY, SQUEEZE, LENNY, ETCH,
    SARGE, WOODY, POTATO, SLINK, HAMM, BO, REX, BUZZ,
];

#[cfg(feature = "chrono")]
mod chrono {
    #![cfg_attr(docsrs, doc(cfg(feature = "chrono")))]

    /// Date past which all the time-related queries will begin to return None.
    const RELEASE_HORIZON: NaiveDate = date!(2025 / 6 / 1).unwrap();

    use super::*;
    use ::chrono::{NaiveDate, Utc};

    impl Release {
        /// Date on which this release was promoted from Debian
        /// `testing` to Debian `stable`. This may be `None` if the release
        /// has not happened yet.
        pub fn released_on(&self) -> Option<&NaiveDate> {
            self.released_on.as_ref()
        }

        /// Date on which this release became no longer supported by the
        /// Debian project as a release.
        ///
        /// There is a best-effort basis project called
        /// "[LTS](https://wiki.debian.org/LTS)" to provide support for a few
        /// years after this date.
        pub fn eol_on(&self) -> Option<&NaiveDate> {
            self.eol_on.as_ref()
        }
    }

    /// Filter the set of all [RELEASES] to just the [Release]s which are or
    /// were supported at the provided time.
    pub fn supported_on(date: &NaiveDate) -> Vec<Release> {
        RELEASES
            .iter()
            .filter(|rel| rel.released_on.is_some())
            .filter(|rel| match &rel.released_on {
                Some(release_date) => release_date < date,
                None => true,
            })
            .filter(|rel| match &rel.eol_on {
                Some(eol_date) => date < eol_date,
                None => true,
            })
            .cloned()
            .collect()
    }

    /// This is only really semi-reliable in the *PAST*. Giving this a date
    /// in the future may or may not result in EXTREME PAIN depending on
    /// what you're doing and how much you know about Debian's release process.
    ///
    /// There is no `guess_release_suites` function for this very reason. The
    /// older this library is the greater a chance this function returns
    /// absoluetly bogus information. Use it wisely.
    ///
    /// As a result, there's a hardcoded date (internally we call this the
    /// "`RELEASE_HORIZON`"), past which this function will begin to return
    /// a `None`, requiring an update and recompile.
    pub fn guess_release_suites_on(date: &NaiveDate) -> Option<[Release; 2]> {
        if *date > RELEASE_HORIZON {
            return None;
        }

        let releases = supported_on(date);
        let stable = releases.first()?;

        // if this fails something very bad has happened.
        let stable_idx = RELEASES.iter().position(|e| e == stable).unwrap();

        if stable_idx <= 1 {
            return None;
        }

        let [testing, stable] = RELEASES
            .into_iter()
            .skip(stable_idx - 1)
            .take(2)
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        Some([testing, stable])
    }

    /// Filter the set of all [RELEASES] to just the [Release]s which are or
    /// were supported at the time of this function call.
    pub fn supported() -> Vec<Release> {
        let today = Utc::now().naive_utc().date();
        supported_on(&today)
    }

    /// Return the list of [Architecture]s supported by all supported stable
    /// releases at the provided time.
    pub fn supported_architectures_on(date: &NaiveDate) -> Vec<Architecture> {
        let mut ret = vec![];
        for arch in supported_on(date)
            .iter()
            .flat_map(|rel| &rel.architectures[..])
        {
            if ret.contains(arch) {
                continue;
            }
            ret.push(arch.clone());
        }
        ret
    }

    /// Return the list of [Architecture]s supported by all supported stable
    /// releases at the time of this function call.
    ///
    /// ```
    /// use deb::release::supported_architectures;
    ///
    /// // Print all supported release multiarch tuples.
    /// println!("{}", supported_architectures()
    ///     .into_iter()
    ///     .map(|arch| arch.multiarch_tuple().unwrap().to_string())
    ///     .collect::<Vec<_>>()
    ///     .join(", "));
    /// ```
    pub fn supported_architectures() -> Vec<Architecture> {
        let today = Utc::now().naive_utc().date();
        supported_architectures_on(&today)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_supported_on() {
            let supported_releases = supported_on(&NaiveDate::from_ymd_opt(2023, 7, 1).unwrap());
            assert_eq!(2, supported_releases.len());
            assert_eq!(vec![BOOKWORM, BULLSEYE], supported_releases);

            let supported_releases = supported_on(&NaiveDate::from_ymd_opt(2012, 6, 26).unwrap());
            assert_eq!(1, supported_releases.len());
            assert_eq!(vec![SQUEEZE], supported_releases);
        }

        #[test]
        fn test_supported_architectures_on() {
            let supported_architectures =
                supported_architectures_on(&NaiveDate::from_ymd_opt(2023, 7, 1).unwrap());

            let mut supported_architectures = supported_architectures
                .into_iter()
                .map(|arch| arch.to_string())
                .collect::<Vec<String>>();

            supported_architectures.sort();

            assert_eq!(
                vec![
                    "amd64", "arm64", "armel", "armhf", "i386", "mips64el", "mipsel", "ppc64el",
                    "s390x"
                ],
                supported_architectures
            );
        }

        #[test]
        fn test_releases_on() {
            assert_eq!(
                Some([TRIXIE, BOOKWORM]),
                guess_release_suites_on(&NaiveDate::from_ymd_opt(2023, 7, 1).unwrap())
            );

            assert_eq!(
                Some([WHEEZY, SQUEEZE]),
                guess_release_suites_on(&NaiveDate::from_ymd_opt(2012, 6, 26).unwrap())
            );

            assert_eq!(
                None,
                guess_release_suites_on(&NaiveDate::from_ymd_opt(1980, 6, 26).unwrap())
            );

            let date_past_horizon = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
            assert!(
                date_past_horizon > RELEASE_HORIZON,
                "update this test case after updating RELEASE_HORIZON"
            );

            assert!(guess_release_suites_on(&date_past_horizon).is_none());
        }
    }
}

#[cfg(feature = "chrono")]
pub use chrono::{
    guess_release_suites_on, supported, supported_architectures, supported_architectures_on,
    supported_on,
};

// vim: foldmethod=marker
