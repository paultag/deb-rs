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

use crate::architecture::Architecture;
use std::borrow::Cow;

///
#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    ///
    pub name: Cow<'static, str>,

    /// Debian release's Version number
    pub version: Cow<'static, str>,

    ///
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
    released_on: date!(1996 / 06 / 16),
    eol_on: date!(1996 / 12 / 12),
    architectures: cow!(&[Architecture::I386]),
};

/// Debian 1.2, Rex
pub const REX: Release = Release {
    name: cow!("rex"),
    version: cow!("1.2"),
    released_on: date!(1996 / 12 / 12),
    eol_on: date!(1997 / 07 / 02),
    architectures: cow!(&[Architecture::I386]),
};

/// Debian 1.3, Bo
///
/// This was the first release to support non-[Architecture::I386]
/// architectures in the stable release -- and a lot of them, too! This
/// version saw the inclusion of [Architecture::M68K], [Architecture::ALPHA],
/// and [Architecture::SPARC].
pub const BO: Release = Release {
    name: cow!("bo"),
    version: cow!("1.3"),
    released_on: date!(1997 / 07 / 02),
    eol_on: date!(1998 / 07 / 24),
    architectures: cow!(&[
        Architecture::I386,
        Architecture::M68K,
        Architecture::ALPHA,
        Architecture::SPARC
    ]),
};

/// Debian 2.0, Hamm
///
/// This release trimmed back on architecture support (a good thing, in
/// retrospect! Maintaining ports is hard!), only supporting
/// [Architecture::I386] and [Architecture::M68K]; with the rest of the
/// architectures from [BO] being only supported in `unstable`.
///
/// This also saw a huge `libc` transition (libc5 to libc6).
pub const HAMM: Release = Release {
    name: cow!("hamm"),
    version: cow!("2.0"),
    released_on: date!(1998 / 06 / 24),
    eol_on: date!(1999 / 03 / 09),

    // Alpha, Sparc, and PowerPC were in unstable.
    architectures: cow!(&[Architecture::I386, Architecture::M68K]),
};

/// Debian 2.1, Slink
///
/// `slink` saw the reintroduction of [Architecture::ALPHA] and
/// [Architecture::SPARC] as supported in the `stable` suite.
pub const SLINK: Release = Release {
    name: cow!("slink"),
    version: cow!("2.1"),
    released_on: date!(1999 / 03 / 09),
    eol_on: date!(2000 / 09 / 30),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::I386,
        Architecture::M68K,
        Architecture::SPARC,
    ]),
};

/// Debian 2.2, Potato
///
/// `potato` grew two new supported [Architecture]s, [Architecture::ARM]
/// and [Architecture::POWERPC].
///
/// This is also the first release featuring `apt`!
pub const POTATO: Release = Release {
    name: cow!("potato"),
    version: cow!("2.2"),
    released_on: date!(2000 / 08 / 15),
    eol_on: date!(2003 / 06 / 30),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::ARM,
        Architecture::I386,
        Architecture::M68K,
        Architecture::POWERPC,
        Architecture::SPARC,
    ]),
};

/// Debian 3.0, Woody
///
/// This was a big one! "Yee-haw! Giddy-up partner! We've got to get this wagon
/// train a-movin'!"
///
/// `woody` grew a bunch of new supported [Architecture]s, [Architecture::HPPA],
/// [Architecture::IA64], [Architecture::MIPS], [Architecture::MIPSEL] and
/// [Architecture::S390].
pub const WOODY: Release = Release {
    name: cow!("woody"),
    version: cow!("3.0"),
    released_on: date!(2002 / 07 / 19),
    eol_on: date!(2006 / 06 / 30),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::ARM,
        Architecture::HPPA,
        Architecture::I386,
        Architecture::IA64,
        Architecture::M68K,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::SPARC,
    ]),
};

/// Debian 3.1, Sarge
pub const SARGE: Release = Release {
    // paultag's first Debian install!
    name: cow!("sarge"),
    version: cow!("3.1"),
    released_on: date!(2005 / 06 / 06),
    eol_on: date!(2008 / 03 / 31),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::ARM,
        Architecture::HPPA,
        Architecture::I386,
        Architecture::IA64,
        Architecture::M68K,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::SPARC,
    ]),
};

/// Debian 4.0, Etch
///
/// This was the first `stable` release with full official support for the
/// brand new [Architecture::AMD64] [Architecture]! It was also the last
/// release with official [Architecture::M68K] support, being dropped before
/// release.
pub const ETCH: Release = Release {
    name: cow!("etch"),
    version: cow!("4.0"),
    released_on: date!(2007 / 04 / 08),
    eol_on: date!(2012 / 02 / 06),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::AMD64,
        Architecture::ARM,
        Architecture::HPPA,
        Architecture::I386,
        Architecture::IA64,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::SPARC,
    ]),
};

/// Debian 5.0, Lenny
///
/// The new arm architecture [Architecture::ARMEL] was officially supported
/// as of this release. This also deprecated the [Architecture::ARM] support.
pub const LENNY: Release = Release {
    name: cow!("lenny"),
    version: cow!("5.0"),
    released_on: date!(2009 / 02 / 14),
    eol_on: date!(2012 / 02 / 06),
    architectures: cow!(&[
        Architecture::ALPHA,
        Architecture::AMD64,
        Architecture::ARMEL,
        Architecture::HPPA,
        Architecture::I386,
        Architecture::IA64,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::SPARC,
    ]),
};

/// Debian 6.0, Squeeze
///
/// The [Architecture::HPPA] [Architecture] was dropped in this release.
///
/// This release grew two new technical previews --
/// [Architecture::KFREEBSD_AMD64] and [Architecture::KFREEBSD_I386] --
/// a port of the GNU userland, libc and Debian software ecosystem to run under
/// the FreeBSD kernel. These were not officially supported, however.
pub const SQUEEZE: Release = Release {
    name: cow!("squeeze"),
    version: cow!("6.0"),
    released_on: date!(2011 / 02 / 06),
    eol_on: date!(2014 / 05 / 31),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARMEL,
        Architecture::I386,
        Architecture::IA64,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::SPARC,
    ]),
};

/// Debian 7, Wheezy
///
/// Two new [Architecture]s here! `wheezy` was ported and offically supported
/// on [Architecture::ARMHF] and [Architecture::S390X].
pub const WHEEZY: Release = Release {
    // paultag's first Debian release as a DD
    name: cow!("wheezy"),
    version: cow!("7"),
    released_on: date!(2013 / 05 / 04),
    eol_on: date!(2016 / 04 / 25),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::IA64,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::S390,
        Architecture::S390X,
        Architecture::SPARC,
    ]),
};

/// Debian 8, Jessie
///
/// Yeee haw! [Architecture::ARM64] and [Architecture::PPC64EL] were added
/// as supported, and [Architecture::S390], [Architecture::SPARC] and
/// [Architecture::IA64] were dropped.
pub const JESSIE: Release = Release {
    name: cow!("jessie"),
    version: cow!("8"),
    released_on: date!(2015 / 04 / 25),
    eol_on: date!(2018 / 06 / 17),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARM64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::MIPS,
        Architecture::MIPSEL,
        Architecture::POWERPC,
        Architecture::PPC64EL,
        Architecture::S390X,
    ]),
};

/// Debian 9, Stretch
///
/// [Release Announcement](https://lists.debian.org/debian-announce/2017/msg00003.html)
pub const STRETCH: Release = Release {
    name: cow!("stretch"),
    version: cow!("9"),
    released_on: date!(2017 / 06 / 17),
    eol_on: date!(2020 / 07 / 18),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARM64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::MIPS,
        Architecture::MIPS64EL,
        Architecture::MIPSEL,
        Architecture::PPC64EL,
        Architecture::S390X,
    ]),
};

/// Debian 10, Buster
///
/// [Release Announcement](https://lists.debian.org/debian-announce/2019/msg00003.html)
pub const BUSTER: Release = Release {
    name: cow!("buster"),
    version: cow!("10"),
    released_on: date!(2019 / 07 / 06),
    eol_on: date!(2022 / 09 / 10),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARM64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::MIPS,
        Architecture::MIPS64EL,
        Architecture::MIPSEL,
        Architecture::PPC64EL,
        Architecture::S390X,
    ]),
};

/// Debian 11, Bullseye
pub const BULLSEYE: Release = Release {
    name: cow!("bullseye"),
    version: cow!("11"),
    released_on: date!(2021 / 08 / 14),
    eol_on: date!(2024 / 08 / 14),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARM64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::MIPS64EL,
        Architecture::MIPSEL,
        Architecture::PPC64EL,
        Architecture::S390X,
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
    released_on: date!(2023 / 06 / 10),
    eol_on: date!(2026 / 06 / 10),
    architectures: cow!(&[
        Architecture::AMD64,
        Architecture::ARM64,
        Architecture::ARMEL,
        Architecture::ARMHF,
        Architecture::I386,
        Architecture::MIPS64EL,
        Architecture::MIPSEL,
        Architecture::PPC64EL,
        Architecture::S390X,
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
/// [Architecture::RISCV64] [Architecture]. The [Architecture::I386] port
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
    version: cow!("13"),
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
    FORKY, TRIXIE, BOOKWORM, BULLSEYE, BUSTER, STRETCH, JESSIE, WHEEZY, SQUEEZE, LENNY, ETCH,
    SARGE, WOODY, POTATO, SLINK, HAMM, BO, REX, BUZZ,
];

#[cfg(feature = "chrono")]
mod chrono {
    // #![cfg_attr(docsrs, doc(cfg(feature = "chrono")))]

    use super::{Architecture, Release, RELEASES};
    use ::chrono::NaiveDate;

    impl Release {
        /// Date on which this release was promoted from Debian
        /// `testing to Debian `stable`. This may be `None` if the release
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
    pub fn supported_on(today: &NaiveDate) -> Vec<Release> {
        RELEASES
            .iter()
            .filter(|rel| rel.released_on.is_some())
            .filter(|rel| match &rel.eol_on {
                Some(eol_date) => today < eol_date,
                None => true,
            })
            .cloned()
            .collect()
    }

    /// Filter the set of all [RELEASES] to just the [Release]s which are or
    /// were supported at the time of this function call.
    pub fn supported() -> Vec<Release> {
        let today = chrono::Utc::now().naive_utc().date();
        supported_on(&today)
    }

    /// Return the list of [Architecture]s supported by all supported stable
    /// releases at the provided time.
    pub fn supported_architectures_on(today: &NaiveDate) -> Vec<Architecture> {
        let mut ret = vec![];
        for arch in supported_on(&today)
            .iter()
            .map(|rel| &rel.architectures[..])
            .flatten()
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
    /// use deb::releases;
    ///
    /// // Print all supported release multiarch tuples.
    /// println!("{}", releases::supported_architectures()
    ///     .into_iter()
    ///     .map(|arch| arch.multiarch_tuple().unwrap().to_string())
    ///     .collect::<Vec<_>>()
    ///     .join(", "));
    /// ```
    pub fn supported_architectures() -> Vec<Architecture> {
        let today = chrono::Utc::now().naive_utc().date();
        supported_architectures_on(&today)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::releases;

        #[test]
        fn test_supported_on() {
            let supported_releases = supported_on(&NaiveDate::from_ymd_opt(2023, 07, 01).unwrap());
            assert_eq!(2, supported_releases.len());
            assert_eq!(
                vec![releases::BOOKWORM, releases::BULLSEYE],
                supported_releases
            );
        }

        #[test]
        fn test_supported_architectures_on() {
            let supported_architectures =
                supported_architectures_on(&NaiveDate::from_ymd_opt(2023, 07, 01).unwrap());

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
    }
}

#[cfg(feature = "chrono")]
pub use chrono::{supported, supported_architectures, supported_architectures_on, supported_on};

// vim: foldmethod=marker