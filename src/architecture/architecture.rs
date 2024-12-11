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
use std::str::FromStr;

/// Debian architecture name. This is something like `arm64`,
/// `kfreebsd-amd64` or `mips64el`. This can be parsed from
/// (or converted back into) a String.
///
/// [Architecture] strings are used through Debian infrastructure to target
/// a specific CPU ISA baseline and kernel type. They can be found in Debian
/// packages, archive directories, or even in `dpkg` cmdline invocations.
///
/// ```
/// use deb::architecture::Architecture;
///
/// // Prints `arm64`
/// println!("{}", Architecture::Arm64);
///
/// let arch: Architecture = "amd64".parse().unwrap();
/// // Prints `amd64`
/// println!("{}", arch);
/// ```
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Architecture {
    /// Special "Any" Architecture -- valid to be build on any Debian
    /// architecture.
    Any,

    /// Special "All" Architecture -- valid for all architectures, things like
    /// text files or other files that do not rely on the host CPU ISA.
    All,

    /// Special "Source" Architecture -- this is source code to a binary
    /// package.
    Source,

    /// Debian `alpha` arch.
    Alpha,

    /// Debian `amd64` arch.
    Amd64,

    /// Debian `arc` arch.
    Arc,

    /// Debian `arm` arch.
    Arm,

    /// Debian `arm64` arch.
    Arm64,

    /// Debian `arm64ilp32` arch.
    Arm64Ilp32,

    /// Debian `armel` arch.
    Armel,

    /// Debian `armhf` arch.
    Armhf,

    /// Debian `hppa` arch.
    Hppa,

    /// Debian `hurd-i386` arch.
    HurdI386,

    /// Debian `hurd-amd64` arch.
    HurdAmd64,

    /// Debian `i386` arch.
    I386,

    /// Debian `ia64` arch.
    Ia64,

    /// Debian `kfreebsd-amd64` arch.
    KFreeBsdAmd64,

    /// Debian `kfreebsd-i386` arch.
    KFreeBsdI386,

    /// Debian `loong64` arch.
    Loong64,

    /// Debian `m68k` arch.
    M68k,

    /// Debian `mips` arch.
    Mips,

    /// Debian `mipsel` arch.
    Mipsel,

    /// Debian `mips64` arch.
    Mips64,

    /// Debian `mips64el` arch.
    Mips64el,

    /// Debian `mipsn32` arch.
    Mipsn32,

    /// Debian `mipsn32el` arch.
    Mipsn32el,

    /// Debian `mipsr6` arch.
    MipsR6,

    /// Debian `mipsr6el` arch.
    MipsR6el,

    /// Debian `mips64r6` arch.
    Mips64R6,

    /// Debian `mips64r6el` arch.
    Mips64R6el,

    /// Debian `mipsn32r6` arch.
    Mipsn32R6,

    /// Debian `mipsn32r6el` arch.
    Mipsn32R6el,

    /// Debian `powerpc` arch.
    PowerPc,

    /// Debian `powerpcspe` arch.
    PowerPcSpe,

    /// Debian `ppc64` arch.
    Ppc64,

    /// Debian `ppc64el` arch.
    Ppc64el,

    /// Debian `riscv64` arch.
    RiscV64,

    /// Debian `s390` arch.
    S390,

    /// Debian `s390x` arch.
    S390X,

    /// Debian `sh4` arch.
    Sh4,

    /// Debian `sparc` arch.
    Sparc,

    /// Debian `sparc64` arch.
    Sparc64,

    /// Debian `uefi-amd64` arch.
    UefiAmd64,

    /// Debian `uefi-arm64` arch.
    UefiArm64,

    /// Debian `uefi-armhf` arch.
    UefiArmhf,

    /// Debian `uefi-i386` arch.
    UefiI386,

    /// Debian `x32` arch.
    X32,

    /// Other Debian arch not covered by this enum at the time of its
    /// last update.
    Other(String),
}

/// Error conditions which may be encountered when parsing a String
/// into an [Architecture].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// Returned when the string provided to [Architecture] is empty.
    Empty,
}
crate::errors::error_enum!(Error);

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

impl Architecture {
    /// Return true if the Architecture has a specific special meaning.
    pub const fn is_special(&self) -> bool {
        matches!(self, Self::Any | Self::All | Self::Source)
    }

    /// Return true if `self` is effectively the `other` [Architecture]
    /// provided.
    ///
    /// | `self`                     | `other`                    | value                  |
    /// | -------------------------- | -------------------------- | ---------------------- |
    /// | [Architecture::Amd64]      | [Architecture::Amd64]      | `true`                 |
    /// | [Architecture::Amd64]      | [Architecture::Arm64]      | `false`                |
    /// | [Architecture::Any]        | [Architecture::Arm64]      | `true`                 |
    /// | [Architecture::Arm64]      | [Architecture::Any]        | `false`                |
    /// | [Architecture::All]        | [Architecture::Arm64]      | `false`                |
    /// | [Architecture::Source]     | [Architecture::Arm64]      | `false`                |
    ///
    pub fn is(&self, other: &Architecture) -> bool {
        if self == other {
            return true;
        }
        if matches!(other, Self::Any) {
            if self.is_special() {
                return false;
            }
            return true;
        }
        false
    }
}

macro_rules! arch_table_multiarch_tuple {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
impl Architecture {
    /// Return the [multiarch::Tuple] for the Debian [Architecture].
    pub fn multiarch_tuple(&self) -> Option<multiarch::Tuple> {
        Some(match self {
            $( $arch => $tuple, )*
            _ => return None,
        })
    }
}

impl multiarch::Tuple {
    /// Return the [Architecture] for a debian [multiarch::Tuple], if it
    /// exists. Not all [multiarch::Tuple] values have a matching Debian [Architecture].
    pub fn arch(&self) -> Option<Architecture> {
        $(
            if *self == $tuple {
                return Some($arch);
            }
        )*
        None
    }
}
    }
}

macro_rules! arch_table_as_str {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
impl Architecture {
    /// Return the [Architecture] as our conventional string representation.
    pub fn as_str(&self) -> &str {
        match self {
            $( $arch => $name, )*
            Self::Any => "any",
            Self::All => "all",
            Self::Source => "source",
            Self::Other(v) => v.as_str(),
        }
    }
}
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

macro_rules! arch_table_from_str {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
impl FromStr for Architecture {
    type Err = Error;

    fn from_str(arch: &str) -> Result<Self, Error> {
        Ok(match arch {
            "" => return Err(Error::Empty),
            "any" => Architecture::Any,
            "all" => Architecture::All,
            "source" => Architecture::Source,
            $( $name => $arch, )*
            _ => Architecture::Other(arch.to_owned()),
        })
    }
}
    }
}

#[allow(unused_macros)]
macro_rules! arch_table_tests {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
$(
    #[test]
    fn $id() {
        let arch: Architecture = $name.parse().unwrap();
        assert_eq!(arch.multiarch_tuple().unwrap(), $tuple);
        assert_eq!(arch.as_str(), $name);
    }
)*
    };
}

#[allow(unused_macros)]
macro_rules! arch_table_tests_for_multiarch {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
$(
    #[test]
    fn $id() {
        let tuple_str = $tuple.to_string();
        assert_eq!($tuple, tuple_str.parse().unwrap());
        assert_eq!($tuple.arch().unwrap(), $arch);
    }
)*
    };
}

macro_rules! arch_table {
    ( $( ( $id:ident, $name:expr, $arch:path, $tuple:expr ) ),* ) => {
        arch_table_multiarch_tuple!($( ($id, $name, $arch, $tuple) ),*);
        arch_table_from_str!($( ($id, $name, $arch, $tuple) ),*);
        arch_table_as_str!($( ($id, $name, $arch, $tuple) ),*);

        #[cfg(test)]
        mod arch_tests {
            use super::*;
            arch_table_tests!($( ($id, $name, $arch, $tuple) ),*);
        }

        #[cfg(test)]
        mod multiarch_tests {
            use super::*;
            arch_table_tests_for_multiarch!($( ($id, $name, $arch, $tuple) ),*);
        }
    };
}

arch_table!(
    (
        alpha,
        "alpha",
        Architecture::Alpha,
        simple_multiarch!(Alpha)
    ),
    (
        amd64,
        "amd64",
        Architecture::Amd64,
        simple_multiarch!(X86_64)
    ),
    (arc, "arc", Architecture::Arc, simple_multiarch!(Arc)),
    (arm, "arm", Architecture::Arm, simple_multiarch!(Arm)),
    (
        arm64,
        "arm64",
        Architecture::Arm64,
        simple_multiarch!(Aarch64)
    ),
    (
        arm64ilp32,
        "arm64ilp32",
        Architecture::Arm64Ilp32,
        simple_multiarch!(Aarch64 - Linux - "gnu_ilp32")
    ),
    (
        armel,
        "armel",
        Architecture::Armel,
        simple_multiarch!(Arm - Linux - "gnueabi")
    ),
    (
        armhf,
        "armhf",
        Architecture::Armhf,
        simple_multiarch!(Arm - Linux - "gnueabihf")
    ),
    (hppa, "hppa", Architecture::Hppa, simple_multiarch!(Hppa)),
    (
        hurd_i386,
        "hurd-i386",
        Architecture::HurdI386,
        simple_multiarch!(I386 - Hurd - "gnu")
    ),
    (
        hurd_amd64,
        "hurd-amd64",
        Architecture::HurdAmd64,
        simple_multiarch!(X86_64 - Hurd - "gnu")
    ),
    (i386, "i386", Architecture::I386, simple_multiarch!(I386)),
    (ia64, "ia64", Architecture::Ia64, simple_multiarch!(Ia64)),
    (
        kfreebsd_amd64,
        "kfreebsd-amd64",
        Architecture::KFreeBsdAmd64,
        simple_multiarch!(X86_64 - FreeBSD - "gnu")
    ),
    (
        kfreebsd_i386,
        "kfreebsd-i386",
        Architecture::KFreeBsdI386,
        simple_multiarch!(I386 - FreeBSD - "gnu")
    ),
    (
        loong64,
        "loong64",
        Architecture::Loong64,
        simple_multiarch!(Loongarch64)
    ),
    (m68k, "m68k", Architecture::M68k, simple_multiarch!(M68k)),
    (mips, "mips", Architecture::Mips, simple_multiarch!(Mips)),
    (
        mipsel,
        "mipsel",
        Architecture::Mipsel,
        simple_multiarch!(Mipsel)
    ),
    (
        mips64,
        "mips64",
        Architecture::Mips64,
        simple_multiarch!(Mips64)
    ),
    (
        mips64el,
        "mips64el",
        Architecture::Mips64el,
        simple_multiarch!(Mips64el - Linux - "gnuabi64")
    ),
    (
        mipsn32,
        "mipsn32",
        Architecture::Mipsn32,
        simple_multiarch!(Mips64 - Linux - "gnuabin32")
    ),
    (
        mipsn32el,
        "mipsn32el",
        Architecture::Mipsn32el,
        simple_multiarch!(Mips64el - Linux - "gnuabin32")
    ),
    (
        mipsr6,
        "mipsr6",
        Architecture::MipsR6,
        simple_multiarch!(MipsIsa32r6)
    ),
    (
        mipsr6el,
        "mipsr6el",
        Architecture::MipsR6el,
        simple_multiarch!(MipsIsa32r6el)
    ),
    (
        mips64r6,
        "mips64r6",
        Architecture::Mips64R6,
        simple_multiarch!(MipsIsa64r6 - Linux - "gnuabi64")
    ),
    (
        mips64r6el,
        "mips64r6el",
        Architecture::Mips64R6el,
        simple_multiarch!(MipsIsa64r6el - Linux - "gnuabi64")
    ),
    (
        mipsn32r6,
        "mipsn32r6",
        Architecture::Mipsn32R6,
        simple_multiarch!(MipsIsa64r6 - Linux - "gnuabin32")
    ),
    (
        mipsn32r6el,
        "mipsn32r6el",
        Architecture::Mipsn32R6el,
        simple_multiarch!(MipsIsa64r6el - Linux - "gnuabin32")
    ),
    (
        powerpc,
        "powerpc",
        Architecture::PowerPc,
        simple_multiarch!(PowerPc)
    ),
    (
        powerpcspe,
        "powerpcspe",
        Architecture::PowerPcSpe,
        simple_multiarch!(PowerPc - Linux - "gnuspe")
    ),
    (
        ppc64,
        "ppc64",
        Architecture::Ppc64,
        simple_multiarch!(PowerPc64)
    ),
    (
        ppc64el,
        "ppc64el",
        Architecture::Ppc64el,
        simple_multiarch!(PowerPc64le)
    ),
    (
        riscv64,
        "riscv64",
        Architecture::RiscV64,
        simple_multiarch!(RiscV64)
    ),
    (s390, "s390", Architecture::S390, simple_multiarch!(S390)),
    (
        s390x,
        "s390x",
        Architecture::S390X,
        simple_multiarch!(S390X)
    ),
    (sh4, "sh4", Architecture::Sh4, simple_multiarch!(Sh4)),
    (
        sparc,
        "sparc",
        Architecture::Sparc,
        simple_multiarch!(Sparc)
    ),
    (
        sparc64,
        "sparc64",
        Architecture::Sparc64,
        simple_multiarch!(Sparc64)
    ),
    (
        uefi_amd64,
        "uefi-amd64",
        Architecture::UefiAmd64,
        simple_multiarch!(X86_64 - Uefi - "uefi")
    ),
    (
        uefi_arm64,
        "uefi-arm64",
        Architecture::UefiArm64,
        simple_multiarch!(Aarch64 - Uefi - "uefi")
    ),
    (
        uefi_armhf,
        "uefi-armhf",
        Architecture::UefiArmhf,
        simple_multiarch!(Arm - Uefi - "uefi")
    ),
    (
        uefi_i386,
        "uefi-i386",
        Architecture::UefiI386,
        simple_multiarch!(I386 - Uefi - "uefi")
    ),
    (
        x32,
        "x32",
        Architecture::X32,
        simple_multiarch!(X86_64 - Linux - "gnux32")
    )
);

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

            assert_eq!(Architecture::Amd64, test.arch);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_is_implementation {
        ($name:ident, $left:ident is $right:ident == $val:expr) => {
            #[test]
            fn $name() {
                assert!($val == Architecture::$left.is(&Architecture::$right));
            }
        };
    }

    check_is_implementation!(is_simple_amd64_amd64, Amd64 is Amd64  == true);
    check_is_implementation!(is_simple_amd64_arm64, Amd64 is Arm64  == false);
    check_is_implementation!(is_simple_amd64_any,   Amd64 is Any    == true);

    check_is_implementation!(is_simple_any_amd64,  Any is Amd64  == false);
    check_is_implementation!(is_simple_any_all,    Any is All    == false);
    check_is_implementation!(is_simple_any_source, Any is Source == false);
    check_is_implementation!(is_simple_any_any,    Any is Any    == true);

    check_is_implementation!(is_simple_all_any,    All is Any    == false);
    check_is_implementation!(is_simple_all_source, All is Source == false);
    check_is_implementation!(is_simple_all_amd64,  All is Amd64  == false);
    check_is_implementation!(is_simple_all_all,    All is All    == true);

    check_is_implementation!(is_simple_source_any,    Source is Any    == false);
    check_is_implementation!(is_simple_source_source, Source is Source == true);
    check_is_implementation!(is_simple_source_amd64,  Source is Amd64  == false);
    check_is_implementation!(is_simple_source_all,    Source is All    == false);

    #[test]
    fn parse_from_string_empty() {
        assert!("".parse::<Architecture>().is_err());
    }

    #[test]
    fn parse_special() {
        assert_eq!(Architecture::Any, "any".parse::<Architecture>().unwrap());
        assert_eq!(Architecture::All, "all".parse::<Architecture>().unwrap());
        assert_eq!(
            Architecture::Source,
            "source".parse::<Architecture>().unwrap()
        );

        assert_eq!("any", Architecture::Any.to_string());
        assert_eq!("all", Architecture::All.to_string());
        assert_eq!("source", Architecture::Source.to_string());

        assert!(Architecture::Any.is_special());
        assert!(Architecture::All.is_special());
        assert!(Architecture::Source.is_special());
    }

    #[test]
    fn parse_from_string_unknown() {
        assert_eq!(
            Architecture::Other("notarealarch".to_owned()),
            "notarealarch".parse::<Architecture>().unwrap()
        );
    }
}

// vim: foldmethod=marker
