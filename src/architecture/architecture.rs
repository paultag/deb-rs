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

use super::multiarch;
use std::{borrow::Cow, str::FromStr};

/// [Architecture] strings are used through Debian infrastructure to target
/// a specific CPU ISA baseline and kernel type.
///
/// They can be found in Debian packages, archive directories, `apt` output,
/// or even in `dpkg` cmdline invocations.
///
/// ```
/// use deb::architecture::Architecture;
///
/// // Prints `arm64`
/// println!("{}", Architecture::ARM64);
///
/// let arch: Architecture = "amd64".parse().unwrap();
/// assert_eq!(Architecture::AMD64, arch);
/// ```
///
/// # Note ♫
///
/// This is not to be confused with a [multiarch::Tuple], which is similar
/// visually, but entirely different.
#[derive(Clone, Debug, PartialEq)]
pub struct Architecture {
    abi: Cow<'static, str>,

    /// System libc implementation.
    libc: Cow<'static, str>,

    /// System operating system kernel.
    os: Cow<'static, str>,

    /// System CPU type.
    cpu: Cow<'static, str>,
}

/// Error conditions which may be encountered when parsing a String into an
/// [Architecture].
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Returned when the string provided to [Architecture] is empty.
    Empty,

    /// Architecture is malformed in some way.
    ///
    /// Something was so broken there's no more specific error about this.
    /// This Error state will be fixed over time, and hopefully removed.
    /// The Architecture string format is described more precisely on
    /// [Architecture].
    ///
    /// # Note ♫
    ///
    /// Be sure that you're not trying to parse a [multiarch::Tuple].
    Malformed,
}
crate::errors::error_enum!(Error);

impl Architecture {
    /// Create a new [Architecture] from the constituent parts.
    ///
    /// This is *NOT* a [multiarch::Tuple].
    pub fn from_parts(abi: &str, libc: &str, os: &str, cpu: &str) -> Result<Self, Error> {
        if abi.contains('-') || libc.contains('-') || os.contains('-') || cpu.contains('-') {
            return Err(Error::Malformed);
        }

        Ok(Self {
            abi: abi.to_lowercase().into(),
            libc: libc.to_lowercase().into(),
            os: os.to_lowercase().into(),
            cpu: cpu.to_lowercase().into(),
        })
    }

    /// Return `true` if any part of this [Architecture] is "any".
    pub fn is_wildcard(&self) -> bool {
        self.abi == "any" || self.libc == "any" || self.os == "any" || self.cpu == "any"
    }

    /// Compare two full [Architecture] values to determine if
    /// the other [Architecture] matches our own.
    pub fn is(&self, other: &Architecture) -> bool {
        // We only check if we're a source or all special arch here; since
        // we want to escape the any glob; but we'll let any glob against
        // any why not.
        if *self == Self::SOURCE || *self == Self::ALL {
            return self == other;
        }

        fn compare(left: &str, right: &str) -> bool {
            if right == "any" {
                return true;
            }
            left == right
        }

        if !compare(&self.abi, &other.abi)
            || !compare(&self.libc, &other.libc)
            || !compare(&self.os, &other.os)
            || !compare(&self.cpu, &other.cpu)
        {
            return false;
        }

        true
    }

    /// Parse a special "wildcard" arch string, an arch string which has
    /// an "any" in it. The arch string must have `any` in it to pass
    /// through this function.
    fn wildcard_from_str(tuple: &str) -> Result<Self, Error> {
        if tuple.is_empty() {
            return Err(Error::Malformed);
        }

        if !tuple.contains("any") {
            return Err(Error::Malformed);
        }

        let chunks: Vec<&str> = tuple.split('-').collect();
        Ok(match chunks[..] {
            [cpu] => Self::from_parts("any", "any", "any", cpu)?,
            [os, cpu] => Self::from_parts("any", "any", os, cpu)?,
            [libc, os, cpu] => Self::from_parts("any", libc, os, cpu)?,
            [abi, libc, os, cpu] => Self::from_parts(abi, libc, os, cpu)?,
            _ => return Err(Error::Malformed),
        })
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Architecture;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Architecture {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for Architecture {
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
        fn serde_architecture() {
            #[derive(Clone, Debug, PartialEq, Deserialize)]
            struct Test {
                #[serde(rename = "Arch")]
                arch: Architecture,
            }

            let test: Test = control::de::from_reader(&mut BufReader::new(Cursor::new(
                "\
Arch: amd64
",
            )))
            .unwrap();

            assert_eq!(Architecture::AMD64, test.arch);
        }
    }
}

macro_rules! match_tuple_to_parts {
    ($chunks:expr) => {
        match $chunks {
            ["eabi", "uclibc", "linux", "arm"] => vec!["uclibc", "linux", "armel"],
            ["eabihf", "musl", "linux", "arm"] => vec!["musl", "linux", "armhf"],
            ["eabihf", "gnu", "linux", "arm"] => vec!["armhf"],
            ["eabi", "gnu", "linux", "arm"] => vec!["armel"],
            ["abin32", "gnu", "linux", "mips64r6el"] => vec!["mipsn32r6el"],
            ["abin32", "gnu", "linux", "mips64r6"] => vec!["mipsn32r6"],
            ["abin32", "gnu", "linux", "mips64el"] => vec!["mipsn32el"],
            ["abin32", "gnu", "linux", "mips64"] => vec!["mipsn32"],
            ["abi64", "gnu", "linux", "mips64r6el"] => vec!["mips64r6el"],
            ["abi64", "gnu", "linux", "mips64r6"] => vec!["mips64r6"],
            ["abi64", "gnu", "linux", "mips64el"] => vec!["mips64el"],
            ["abi64", "gnu", "linux", "mips64"] => vec!["mips64"],
            ["spe", "gnu", "linux", "powerpc"] => vec!["powerpcspe"],
            ["x32", "gnu", "linux", "amd64"] => vec!["x32"],
            ["base", "tos", "mint", "m68k"] => vec!["mint", "m68k"],
            ["base", "gnu", "hurd", cpu] => vec!["hurd", cpu],
            ["base", "bsd", "freebsd", cpu] => vec!["freebsd", cpu],
            ["base", "gnu", "kfreebsd", cpu] => vec!["kfreebsd", cpu],
            ["base", "bsd", "openbsd", cpu] => vec!["openbsd", cpu],
            ["base", "bsd", "netbsd", cpu] => vec!["netbsd", cpu],
            ["base", "bsd", "darwin", cpu] => vec!["darwin", cpu],
            ["base", "gnu", "kopensolaris", cpu] => vec!["kopensolaris", cpu],
            ["base", "bsd", "dragonflybsd", cpu] => vec!["dragonflybsd", cpu],
            ["base", "sysv", "aix", cpu] => vec!["aix", cpu],
            ["base", "sysv", "solaris", cpu] => vec!["solaris", cpu],
            ["base", "musl", "linux", cpu] => vec!["musl", "linux", cpu],
            ["base", "uclibc", "linux", cpu] => vec!["uclibc", "linux", cpu],
            ["base", "gnu", "linux", cpu] => vec![cpu],
            ["source", "", "", ""] => vec!["source"],
            ["all", "", "", ""] => vec!["all"],
            [abi, libc, os, cpu] => vec![abi, libc, os, cpu],
        }
    };
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.is_special() {
            if *self == Self::ALL {
                return write!(f, "all");
            }
            if *self == Self::SOURCE {
                return write!(f, "source");
            }
        }

        write!(
            f,
            "{}",
            match_tuple_to_parts!([
                self.abi.as_ref(),
                self.libc.as_ref(),
                self.os.as_ref(),
                self.cpu.as_ref()
            ])
            .join("-")
            .trim_start_matches("any-")
        )
    }
}

macro_rules! match_arch_to_tuple {
    ($chunks:expr) => {
        Ok(match $chunks {
            ["uclibc", "linux", "armel"] => ["eabi", "uclibc", "linux", "arm"],
            ["musl", "linux", "armhf"] => ["eabihf", "musl", "linux", "arm"],
            ["armhf"] => ["eabihf", "gnu", "linux", "arm"],
            ["armel"] => ["eabi", "gnu", "linux", "arm"],
            ["mipsn32r6el"] => ["abin32", "gnu", "linux", "mips64r6el"],
            ["mipsn32r6"] => ["abin32", "gnu", "linux", "mips64r6"],
            ["mipsn32el"] => ["abin32", "gnu", "linux", "mips64el"],
            ["mipsn32"] => ["abin32", "gnu", "linux", "mips64"],
            ["mips64r6el"] => ["abi64", "gnu", "linux", "mips64r6el"],
            ["mips64r6"] => ["abi64", "gnu", "linux", "mips64r6"],
            ["mips64el"] => ["abi64", "gnu", "linux", "mips64el"],
            ["mips64"] => ["abi64", "gnu", "linux", "mips64"],
            ["powerpcspe"] => ["spe", "gnu", "linux", "powerpc"],
            ["x32"] => ["x32", "gnu", "linux", "amd64"],
            ["mint", "m68k"] => ["base", "tos", "mint", "m68k"],
            ["hurd", cpu] => ["base", "gnu", "hurd", cpu],
            ["freebsd", cpu] => ["base", "bsd", "freebsd", cpu],
            ["kfreebsd", cpu] => ["base", "gnu", "kfreebsd", cpu],
            ["openbsd", cpu] => ["base", "bsd", "openbsd", cpu],
            ["netbsd", cpu] => ["base", "bsd", "netbsd", cpu],
            ["darwin", cpu] => ["base", "bsd", "darwin", cpu],
            ["kopensolaris", cpu] => ["base", "gnu", "kopensolaris", cpu],
            ["dragonflybsd", cpu] => ["base", "bsd", "dragonflybsd", cpu],
            ["aix", cpu] => ["base", "sysv", "aix", cpu],
            ["solaris", cpu] => ["base", "sysv", "solaris", cpu],
            ["musl", "linux", cpu] => ["base", "musl", "linux", cpu],
            ["uclibc", "linux", cpu] => ["base", "uclibc", "linux", cpu],
            [cpu] => ["base", "gnu", "linux", cpu],
            [abi, libc, os, cpu] => [abi, libc, os, cpu],
            _ => return Err(Error::Malformed),
        }
        .try_into()
        // here we can unwrap safely because we know the error states that
        // can happen with `from_parts`.
        .unwrap())
    };
}

impl TryFrom<[&str; 4]> for Architecture {
    type Error = Error;

    fn try_from(from: [&str; 4]) -> Result<Self, Error> {
        let [abi, libc, os, cpu] = from;
        Self::from_parts(abi, libc, os, cpu)
    }
}

impl FromStr for Architecture {
    type Err = Error;

    fn from_str(tuple: &str) -> Result<Self, Self::Err> {
        if tuple.is_empty() {
            return Err(Error::Malformed);
        }

        if tuple.contains("any") {
            return Self::wildcard_from_str(tuple);
        }

        match tuple {
            "source" => return Ok(Self::SOURCE),
            "all" => return Ok(Self::ALL),
            _ => {}
        };

        let chunks: Vec<&str> = tuple.split('-').collect();
        match_arch_to_tuple!(chunks[..])
    }
}

macro_rules! simple_architecture_tuple {
    ($cpu:expr) => {
        simple_architecture_tuple!("base", "gnu", "linux", $cpu)
    };

    ($abi:expr, $libc:expr, $os:expr, $cpu:expr) => {
        Architecture {
            abi: Cow::Borrowed($abi),
            libc: Cow::Borrowed($libc),
            os: Cow::Borrowed($os),
            cpu: Cow::Borrowed($cpu),
        }
    };
}

macro_rules! arch_table_impl_consts {
    ( $( ( $const_name:ident, $doc:expr, $arch:expr ) ),* ) => {
impl Architecture {
$(

    #[doc = $doc]
    pub const $const_name: Architecture = $arch;

)*
}
    }
}

macro_rules! arch_table_multiarch_tuple {
    ( $( ( $arch:expr, $tuple:expr ) ),* ) => {
impl Architecture {
    /// Return the [multiarch::Tuple] for the Debian [Architecture].
    pub fn multiarch_tuple(&self) -> Option<multiarch::Tuple> {
$(
        if self == &$arch {
            return Some($tuple);
        }
)*
        None
    }
}
    };
}

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! arch_table_tests {
    ( $( ( $str:expr, $name:ident, $arch:expr, $tuple:expr ) ),* ) => {
$(
    #[test]
    fn $name() {
        // Check a simple parse matches.
        let a1: Architecture = $str.parse().unwrap();
        assert_eq!($arch, a1);

        // Check a round trip to a string works.
        let a2: Architecture = $arch.to_string().parse().unwrap();
        assert_eq!($arch, a2);
    }
)*
    };
}

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! arch_table_tests_multiarch {
    ( $( ( $str:expr, $name:ident, $arch:expr, $tuple:expr ) ),* ) => {
$(
    #[test]
    fn $name() {
        assert_eq!($tuple, $arch.multiarch_tuple().unwrap());
    }
)*
    };
}

macro_rules! arch_table {
    ( $( ( $str:expr, $name:ident, $doc:expr, $const_name:ident, $arch:expr, $tuple:expr ) ),* ) => {
        arch_table_impl_consts!($( ($const_name, $doc, $arch) ),* );
        arch_table_multiarch_tuple!($( ($arch, $tuple) ),* );

        #[cfg(test)]
        mod arch_tests {
            use super::*;
            arch_table_tests!($( ( $str, $name, $arch, $tuple ) ),*);
        }

        #[cfg(test)]
        mod multiarch_tests {
            use super::*;
            arch_table_tests_multiarch!($( ( $str, $name, $arch, $tuple ) ),*);
        }
    };
}

macro_rules! simple_multiarch {
    ($isa:ident - $syscall_abi:ident - $userland:expr) => {
        multiarch::Tuple {
            instruction_set: multiarch::InstructionSet::$isa,
            syscall_abi: multiarch::SyscallAbi::$syscall_abi,
            userland: $userland.to_owned(),
        }
    };
    ($isa:ident) => {
        multiarch::Tuple {
            instruction_set: multiarch::InstructionSet::$isa,
            syscall_abi: multiarch::SyscallAbi::Linux,
            userland: "gnu".to_owned(),
        }
    };
}

arch_table!(
    // Debian well known arches
    (
        "alpha",
        alpha,
        "Debian's `alpha` arch",
        ALPHA,
        simple_architecture_tuple!("alpha"),
        simple_multiarch!(Alpha)
    ),
    (
        "amd64",
        amd64,
        "Debian's `amd64` arch",
        AMD64,
        simple_architecture_tuple!("amd64"),
        simple_multiarch!(X86_64)
    ),
    (
        "arc",
        arc,
        "Debian's `arc` arch",
        ARC,
        simple_architecture_tuple!("arc"),
        simple_multiarch!(Arc)
    ),
    (
        "arm",
        arm,
        "Debian's `arm` arch",
        ARM,
        simple_architecture_tuple!("arm"),
        simple_multiarch!(Arm)
    ),
    (
        "arm64",
        arm64,
        "Debian's `arm64` arch",
        ARM64,
        simple_architecture_tuple!("arm64"),
        simple_multiarch!(Aarch64)
    ),
    (
        "armel",
        armel,
        "Debian's `armel` arch",
        ARMEL,
        simple_architecture_tuple!("eabi", "gnu", "linux", "arm"),
        simple_multiarch!(Arm - Linux - "gnueabi")
    ),
    (
        "armhf",
        armhf,
        "Debian's `armhf` arch",
        ARMHF,
        simple_architecture_tuple!("eabihf", "gnu", "linux", "arm"),
        simple_multiarch!(Arm - Linux - "gnueabihf")
    ),
    (
        "hppa",
        hppa,
        "Debian's `hppa` arch",
        HPPA,
        simple_architecture_tuple!("hppa"),
        simple_multiarch!(Hppa)
    ),
    (
        "hurd-i386",
        hurd_i386,
        "Debian's `hurd-i386` arch",
        HURD_I386,
        simple_architecture_tuple!("base", "gnu", "hurd", "i386"),
        simple_multiarch!(I386 - Hurd - "gnu")
    ),
    (
        "hurd-amd64",
        hurd_amd64,
        "Debian's `hurd-amd64` arch",
        HURD_AMD64,
        simple_architecture_tuple!("base", "gnu", "hurd", "amd64"),
        simple_multiarch!(X86_64 - Hurd - "gnu")
    ),
    (
        "i386",
        i386,
        "Debian's `i386` arch",
        I386,
        simple_architecture_tuple!("i386"),
        simple_multiarch!(I386)
    ),
    (
        "ia64",
        ia64,
        "Debian's `ia64` arch",
        IA64,
        simple_architecture_tuple!("ia64"),
        simple_multiarch!(Ia64)
    ),
    (
        "kfreebsd-amd64",
        kfreebsd_amd64,
        "Debian's `kfreebsd-amd64` arch",
        KFREEBSD_AMD64,
        simple_architecture_tuple!("base", "gnu", "kfreebsd", "amd64"),
        simple_multiarch!(X86_64 - FreeBSD - "gnu")
    ),
    (
        "kfreebsd-i386",
        kfreebsd_i386,
        "Debian's `kfreebsd-i386` arch",
        KFREEBSD_I386,
        simple_architecture_tuple!("base", "gnu", "kfreebsd", "i386"),
        simple_multiarch!(I386 - FreeBSD - "gnu")
    ),
    (
        "loong64",
        loong64,
        "Debian's `loong64` arch",
        LOONG64,
        simple_architecture_tuple!("loong64"),
        simple_multiarch!(Loongarch64)
    ),
    (
        "m68k",
        m68k,
        "Debian's `m86k` arch",
        M68K,
        simple_architecture_tuple!("m68k"),
        simple_multiarch!(M68k)
    ),
    (
        "mips",
        mips,
        "Debian's `mips` arch",
        MIPS,
        simple_architecture_tuple!("mips"),
        simple_multiarch!(Mips)
    ),
    (
        "mipsel",
        mipsel,
        "Debian's `mipsel` arch",
        MIPSEL,
        simple_architecture_tuple!("mipsel"),
        simple_multiarch!(Mipsel)
    ),
    (
        "mips64",
        mips64,
        "Debian's `mips64` arch",
        MIPS64,
        simple_architecture_tuple!("abi64", "gnu", "linux", "mips64"),
        simple_multiarch!(Mips64)
    ),
    (
        "mips64el",
        mips64el,
        "Debian's `mips64el` arch",
        MIPS64EL,
        simple_architecture_tuple!("abi64", "gnu", "linux", "mips64el"),
        simple_multiarch!(Mips64el - Linux - "gnuabi64")
    ),
    (
        "mipsn32",
        mipsn32,
        "Debian's `mipsn32` arch",
        MIPSN32,
        simple_architecture_tuple!("abin32", "gnu", "linux", "mips64"),
        simple_multiarch!(Mips64 - Linux - "gnuabin32")
    ),
    (
        "mipsn32el",
        mipsn32el,
        "Debian's `mipsn32el` arch",
        MIPSN32EL,
        simple_architecture_tuple!("abin32", "gnu", "linux", "mips64el"),
        simple_multiarch!(Mips64el - Linux - "gnuabin32")
    ),
    (
        "mips64r6",
        mips64r6,
        "Debian's `mips64r6` arch",
        MIPS64R6,
        simple_architecture_tuple!("abi64", "gnu", "linux", "mips64r6"),
        simple_multiarch!(MipsIsa64r6 - Linux - "gnuabi64")
    ),
    (
        "mips64r6el",
        mips64r6el,
        "Debian's `mips64r6el` arch",
        MIPS64R6EL,
        simple_architecture_tuple!("abi64", "gnu", "linux", "mips64r6el"),
        simple_multiarch!(MipsIsa64r6el - Linux - "gnuabi64")
    ),
    (
        "mipsn32r6",
        mipsn32r6,
        "Debian's `mipsn32r6` arch",
        MIPSN32R6,
        simple_architecture_tuple!("abin32", "gnu", "linux", "mips64r6"),
        simple_multiarch!(MipsIsa64r6 - Linux - "gnuabin32")
    ),
    (
        "mipsn32r6el",
        mipsn32r6el,
        "Debian's `mipsn32r6el` arch",
        MIPSN32R6EL,
        simple_architecture_tuple!("abin32", "gnu", "linux", "mips64r6el"),
        simple_multiarch!(MipsIsa64r6el - Linux - "gnuabin32")
    ),
    (
        "powerpc",
        powerpc,
        "Debian's `powerpc` arch",
        POWERPC,
        simple_architecture_tuple!("powerpc"),
        simple_multiarch!(PowerPc)
    ),
    (
        "powerpcspe",
        powerpcspe,
        "Debian's `powerpcspe` arch",
        POWERPCSPE,
        simple_architecture_tuple!("spe", "gnu", "linux", "powerpc"),
        simple_multiarch!(PowerPc - Linux - "gnuspe")
    ),
    (
        "ppc64",
        ppc64,
        "Debian's `ppc64` arch",
        PPC64,
        simple_architecture_tuple!("ppc64"),
        simple_multiarch!(PowerPc64)
    ),
    (
        "ppc64el",
        ppc64el,
        "Debian's `ppc64el` arch",
        PPC64EL,
        simple_architecture_tuple!("ppc64el"),
        simple_multiarch!(PowerPc64le)
    ),
    (
        "riscv64",
        riscv64,
        "Debian's `riscv64` arch",
        RISCV64,
        simple_architecture_tuple!("riscv64"),
        simple_multiarch!(RiscV64)
    ),
    (
        "s390",
        s390,
        "Debian's `s390` arch",
        S390,
        simple_architecture_tuple!("s390"),
        simple_multiarch!(S390)
    ),
    (
        "s390x",
        s390x,
        "Debian's `s390x` arch",
        S390X,
        simple_architecture_tuple!("s390x"),
        simple_multiarch!(S390X)
    ),
    (
        "sh4",
        sh4,
        "Debian's `sh4` arch",
        SH4,
        simple_architecture_tuple!("sh4"),
        simple_multiarch!(Sh4)
    ),
    (
        "sparc",
        sparc,
        "Debian's `sparc` arch",
        SPARC,
        simple_architecture_tuple!("sparc"),
        simple_multiarch!(Sparc)
    ),
    (
        "sparc64",
        sparc64,
        "Debian's `sparc64` arch",
        SPARC64,
        simple_architecture_tuple!("sparc64"),
        simple_multiarch!(Sparc64)
    ),
    (
        "x32",
        x32,
        "Debian's `x32` arch",
        X32,
        simple_architecture_tuple!("x32", "gnu", "linux", "amd64"),
        simple_multiarch!(X86_64 - Linux - "gnux32")
    )
);

impl Architecture {
    /// Debian "source" special "architecture".
    pub const SOURCE: Architecture = Self {
        cpu: Cow::Borrowed("source"),
        libc: Cow::Borrowed(""),
        os: Cow::Borrowed(""),
        abi: Cow::Borrowed(""),
    };

    /// Debian "all" special "architecture".
    pub const ALL: Architecture = Self {
        cpu: Cow::Borrowed("all"),
        libc: Cow::Borrowed(""),
        os: Cow::Borrowed(""),
        abi: Cow::Borrowed(""),
    };

    /// Default "any" glob -- this will match any concrete [Architecture].
    pub const ANY: Architecture = Self {
        cpu: Cow::Borrowed("any"),
        libc: Cow::Borrowed("any"),
        os: Cow::Borrowed("any"),
        abi: Cow::Borrowed("any"),
    };

    /// Return true if the Architecture has a specific special meaning.
    pub fn is_special(&self) -> bool {
        *self == Self::SOURCE || *self == Self::ALL || *self == Self::ANY
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! round_trip {
        ($name:ident, $left:expr, $right:expr) => {
            #[test]
            fn $name() {
                let a: Architecture = $left.parse().unwrap();
                assert_eq!($right, a.to_string());
            }
        };

        ( $( ($name:ident, $left:expr) ),* ) => {
            $(
                round_trip!($name, $left, $left);
            )*
        };
    }

    macro_rules! fails {
        ($name:ident, $tuple:expr) => {
            #[test]
            fn $name() {
                assert!(
                    $tuple.parse::<Architecture>().is_err(),
                    "expected {} to error, it didnt",
                    $tuple,
                );
            }
        };
    }

    #[test]
    fn bad_from_parts() {
        assert!(Architecture::from_parts("a", "b", "c", "d").is_ok());
        assert!(Architecture::from_parts("a", "b", "c-d", "d").is_err());
    }

    fails!(fails_empty, "");
    fails!(fails_5, "any-any-any-any-any");

    round_trip!(rt_any, "any", "any");
    round_trip!(rt_any_any, "any-any", "any");
    round_trip!(rt_any_any_any, "any-any-any", "any");
    round_trip!(rt_any_any_any_any, "any-any-any-any", "any");

    round_trip!(rt_linux_any, "linux-any", "linux-any");
    round_trip!(rt_any_linux_any, "any-linux-any", "linux-any");
    round_trip!(rt_any_any_linux_any, "any-any-linux-any", "linux-any");

    round_trip!(rt_unknown, "dorkus", "dorkus");
    fails!(fails_unknown_pattern, "dorkus-thewise");
    fails!(fails_unknown_pattern_2, "hi-dorkus-thewise");
    round_trip!(
        rt_unknown_all,
        "oh-no-dorkus-thewise",
        "oh-no-dorkus-thewise"
    );

    round_trip!(
        (uclibc_linux_armel, "uclibc-linux-armel"),
        (uclibc_linux_foo, "uclibc-linux-foo"),
        (musl_linux_armhf, "musl-linux-armhf"),
        (musl_linux_bar, "musl-linux-bar"),
        (armhf, "armhf"),
        (armel, "armel"),
        (mipsn32r6el, "mipsn32r6el"),
        (mipsn32r6, "mipsn32r6"),
        (mipsn32el, "mipsn32el"),
        (mipsn32, "mipsn32"),
        (mips64r6el, "mips64r6el"),
        (mips64r6, "mips64r6"),
        (mips64el, "mips64el"),
        (mips64, "mips64"),
        (powerpcspe, "powerpcspe"),
        (x32, "x32"),
        (foo, "foo"),
        (kfreebsd_amd64, "kfreebsd-amd64"),
        (kfreebsd_i386, "kfreebsd-i386"),
        (kopensolaris_amd64, "kopensolaris-amd64"),
        (kopensolaris_i386, "kopensolaris-i386"),
        (hurd_amd64, "hurd-amd64"),
        (hurd_i386, "hurd-i386"),
        (dragonflybsd_amd64, "dragonflybsd-amd64"),
        (freebsd_amd64, "freebsd-amd64"),
        (freebsd_arm, "freebsd-arm"),
        (freebsd_arm64, "freebsd-arm64"),
        (freebsd_i386, "freebsd-i386"),
        (freebsd_powerpc, "freebsd-powerpc"),
        (freebsd_ppc64, "freebsd-ppc64"),
        (freebsd_riscv, "freebsd-riscv"),
        (openbsd_baz, "openbsd-baz"),
        (netbsd_fnord, "netbsd-fnord"),
        (darwin_amd64, "darwin-amd64"),
        (darwin_arm, "darwin-arm"),
        (darwin_arm64, "darwin-arm64"),
        (darwin_i386, "darwin-i386"),
        (darwin_powerpc, "darwin-powerpc"),
        (darwin_ppc64, "darwin-ppc64"),
        (aix_powerpc, "aix-powerpc"),
        (aix_ppc64, "aix-ppc64"),
        (solaris_amd64, "solaris-amd64"),
        (solaris_i386, "solaris-i386"),
        (solaris_sparc, "solaris-sparc"),
        (solaris_sparc64, "solaris-sparc64"),
        (mint_m68k, "mint-m68k")
    );

    macro_rules! check_is_implementation {
        ($name:ident, $left:ident is $right:ident == $val:expr) => {
            #[test]
            fn $name() {
                assert!($val == Architecture::$left.is(&Architecture::$right));
            }
        };
    }

    check_is_implementation!(is_simple_amd64_amd64, AMD64 is AMD64  == true);
    check_is_implementation!(is_simple_amd64_arm64, AMD64 is ARM64  == false);
    check_is_implementation!(is_simple_amd64_any,   AMD64 is ANY    == true);

    check_is_implementation!(is_simple_any_amd64,  ANY is AMD64  == false);
    check_is_implementation!(is_simple_any_all,    ANY is ALL    == false);
    check_is_implementation!(is_simple_any_source, ANY is SOURCE == false);
    check_is_implementation!(is_simple_any_any,    ANY is ANY    == true);

    check_is_implementation!(is_simple_all_any,    ALL is ANY    == false);
    check_is_implementation!(is_simple_all_source, ALL is SOURCE == false);
    check_is_implementation!(is_simple_all_amd64,  ALL is AMD64  == false);
    check_is_implementation!(is_simple_all_all,    ALL is ALL    == true);

    check_is_implementation!(is_simple_source_any,    SOURCE is ANY    == false);
    check_is_implementation!(is_simple_source_source, SOURCE is SOURCE == true);
    check_is_implementation!(is_simple_source_amd64,  SOURCE is AMD64  == false);
    check_is_implementation!(is_simple_source_all,    SOURCE is ALL    == false);

    #[test]
    fn parse_from_string_empty() {
        assert!("".parse::<Architecture>().is_err());
    }

    #[test]
    fn parse_special() {
        assert_eq!(Architecture::ANY, "any".parse::<Architecture>().unwrap());
        assert_eq!(Architecture::ALL, "all".parse::<Architecture>().unwrap());
        assert_eq!(
            Architecture::SOURCE,
            "source".parse::<Architecture>().unwrap()
        );

        assert_eq!("any", Architecture::ANY.to_string());
        assert_eq!("all", Architecture::ALL.to_string());
        assert_eq!("source", Architecture::SOURCE.to_string());

        assert!(Architecture::ANY.is_special());
        assert!(Architecture::ALL.is_special());
        assert!(Architecture::SOURCE.is_special());

        let linux_any: Architecture = "linux-any".parse().unwrap();

        // `any` meets the critera of `linux-any`
        assert!(!Architecture::ANY.is(&linux_any));

        // `linux-any` does not meet the critera of `any`, since it may
        // include things other than linux.
        assert!(linux_any.is(&Architecture::ANY));
    }
}

// vim: foldmethod=marker
