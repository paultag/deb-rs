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

//! Debian-flavored multiarch [Tuple] values describe the host system in a
//! way that's a lot more specific than something like "`amd64`" -- it will
//! tell you what specific CPU instruction set baseline, syscall interface
//! and userland is being targeted.
//!
//! A [Tuple] is made up of three parts -- the first is the [InstructionSet],
//! which is the CPU Instruction Set Architecture (or ISA for short) being
//! targeted. The second is the [SyscallAbi], how a running program interacts
//! with the operating system below it. This is [SyscallAbi::Linux] on Linux
//! based operating systems, [SyscallAbi::FreeBSD] for FreeBSD (and so on).
//! Lastly, there's the `userland` -- which is an opaque string representation
//! of the targeted operating system userland. In cases where the userland
//! and [SyscallAbi] are the same, the userland may be omitted, such as
//! with `x86_64-uefi` or `i386-gnu`.
//!
//! Tuple values are documented on the
//! [Debian Wiki](https://wiki.debian.org/Multiarch/Tuples).
//!
//! ```
//! use deb::architecture::multiarch::Tuple;
//!
//! let tuple: Tuple = "x86_64-linux-gnu".parse().unwrap();
//! // prints "x86_64"
//! println!("{}", tuple.instruction_set);
//!
//! // prints "linux"
//! println!("{}", tuple.syscall_abi);
//!
//! // prints "gnu"
//! println!("{}", tuple.userland);
//! ```
use std::str::FromStr;

/// Syscall API -- the kernel/operating system that the binary is targeted
/// to run against. Using the term "Operating System" here can be ambiguous,
/// as can "kernel", so this struct has an awkward, abet super specific
/// name that is in line with the spec.
///
/// ```
/// use deb::architecture::multiarch::SyscallAbi;
///
/// let abi: SyscallAbi = "linux".parse().unwrap();
/// // prints "Linux"
/// println!("{}", abi);
///
/// // prints "gnu"
/// println!("{}", SyscallAbi::Hurd);
/// ```
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum SyscallAbi {
    /// The binary is intended to run under a Linux kernel.
    Linux,

    /// The binary is intended to run under a HURD lernel.
    Hurd,

    /// The binary is intended to run under a FreeBSD kernel.
    FreeBSD,

    /// The binary is intended to run in the UEFI environment.
    Uefi,

    /// Other system not enumerated here (yet).
    Other(String),
}

impl SyscallAbi {
    /// Return the [SyscallAbi] as our conventional string representation,
    /// as documented by the Debian multiarch Tuple spec.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Linux => "linux",
            Self::Hurd => "gnu",
            Self::FreeBSD => "kfreebsd",
            Self::Uefi => "uefi",
            Self::Other(v) => v.as_str(),
        }
    }
}

impl std::fmt::Display for SyscallAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Error conditions which may be encountered when parsing a String
/// into a [SyscallAbi].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SyscallAbiParseError {
    /// Returned when the string provided to [SyscallAbi] is empty.
    Empty,
}

impl FromStr for SyscallAbi {
    type Err = SyscallAbiParseError;

    fn from_str(abi: &str) -> Result<Self, SyscallAbiParseError> {
        Ok(match abi {
            "" => return Err(SyscallAbiParseError::Empty),
            "linux" => Self::Linux,
            "gnu" => Self::Hurd,
            "kfreebsd" => Self::FreeBSD,
            "uefi" => Self::Uefi,
            _ => Self::Other(abi.to_owned()),
        })
    }
}

/// Error conditions which may be encountered when parsing a String
/// into a [InstructionSet].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InstructionSetParseError {
    /// Returned when the string provided to [InstructionSet] is empty.
    Empty,
}

macro_rules! instruction_set_table_as_str {
    ( $( ( $id:ident, $name:expr, $isa:path ) ),* ) => {
impl InstructionSet {
    /// Return the [InstructionSet] as our conventional string representation,
    /// as documented by the Debian multiarch Tuple spec.
    pub fn as_str(&self) -> &str {
        match self {
            $( $isa => $name, )*
            Self::Other(v) => v.as_str(),
        }
    }
}
    };
}

impl std::fmt::Display for InstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

macro_rules! instruction_set_table_from_str {
    ( $( ( $id:ident, $name:expr, $isa:path ) ),* ) => {
impl FromStr for InstructionSet {
    type Err = InstructionSetParseError;

    fn from_str(isa: &str) -> Result<Self, InstructionSetParseError> {
        Ok(match isa {
            "" => return Err(InstructionSetParseError::Empty),
            $( $name => $isa, )*
            _ => InstructionSet::Other(isa.to_owned()),
        })
    }
}
    };
}

#[allow(unused_macros)]
macro_rules! instruction_set_table_tests {
    ( $( ( $id:ident, $name:expr, $isa:path ) ),* ) => {
$(
    #[test]
    fn $id() {
        let isa: InstructionSet = $name.parse().unwrap();
        assert_eq!($name, isa.as_str());
    }
)*
    };
}

macro_rules! instruction_set_table {
    ( $( ( $id:ident, $name:expr, $isa:path ) ),* ) => {
        instruction_set_table_from_str!($( ($id, $name, $isa) ),*);
        instruction_set_table_as_str!($( ($id, $name, $isa) ),*);

        #[cfg(test)]
        mod arch_tests {
            use super::*;
            instruction_set_table_tests!($( ($id, $name, $isa) ),*);
        }
    };
}

instruction_set_table!(
    (aarch64, "aarch64", InstructionSet::Aarch64),
    (alpha, "alpha", InstructionSet::Alpha),
    (arc, "arc", InstructionSet::Arc),
    (arm, "arm", InstructionSet::Arm),
    (hppa, "hppa", InstructionSet::Hppa),
    (i386, "i386", InstructionSet::I386),
    (ia64, "ia64", InstructionSet::Ia64),
    (loongarch64, "loongarch64", InstructionSet::Loongarch64),
    (m68k, "m68k", InstructionSet::M68k),
    (mips, "mips", InstructionSet::Mips),
    (mipsel, "mipsel", InstructionSet::Mipsel),
    (mips64, "mips64", InstructionSet::Mips64),
    (mips64el, "mips64el", InstructionSet::Mips64el),
    (mipsisa32r6, "mipsisa32r6", InstructionSet::MipsIsa32r6),
    (
        mipsisa32r6el,
        "mipsisa32r6el",
        InstructionSet::MipsIsa32r6el
    ),
    (mipsisa64r6, "mipsisa64r6", InstructionSet::MipsIsa64r6),
    (
        mipsisa64r6el,
        "mipsisa64r6el",
        InstructionSet::MipsIsa64r6el
    ),
    (powerpc, "powerpc", InstructionSet::PowerPc),
    (powerpc64, "powerpc64", InstructionSet::PowerPc64),
    (powerpc64el, "powerpc64el", InstructionSet::PowerPc64le),
    (riscv64, "riscv64", InstructionSet::RiscV64),
    (s390, "s390", InstructionSet::S390),
    (s390x, "s390x", InstructionSet::S390X),
    (sh4, "sh4", InstructionSet::Sh4),
    (sparc, "sparc", InstructionSet::Sparc),
    (sparc64, "sparc64", InstructionSet::Sparc64),
    (x86_64, "x86_64", InstructionSet::X86_64)
);

/// CPU Instruction Set Architecture (ISA) that the binary is targeted to
/// run on. This dicatates what specific opcodes are permitted.
///
/// ```
/// use deb::architecture::multiarch::InstructionSet;
///
/// let isa: InstructionSet = "aarch64".parse().unwrap();
/// // prints "aarch64"
/// println!("{}", isa);
///
/// // prints "x86_64"
/// println!("{}", InstructionSet::X86_64);
/// ```
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum InstructionSet {
    /// 64 bit ARMv8 CPU (arm64).
    Aarch64,

    /// DEC Alpha 64 bit RISC ISA.
    Alpha,

    /// Argonaut RISC Core RISC ISA.
    Arc,

    /// ARM 32 bit RISC ISA.
    Arm,

    /// Hewlett Packard Precision Architecture (HPPA) ISA.
    Hppa,

    /// *The* Intel 386 ISA.
    I386,

    /// Intel Itanium ISA.
    Ia64,

    /// Chinese Government "fork" of the MIPS64 ISA.
    Loongarch64,

    /// Motorola 68000 ISA.
    M68k,

    /// 32 bit RISC ISA ("Microprocessor without Interlocked Pipelined Stages")
    Mips,

    /// 32 bit RISC ISA ("Microprocessor without Interlocked Pipelined Stages"),
    /// but little endian.
    Mipsel,

    /// 64 bit RISC ISA ("Microprocessor without Interlocked Pipelined Stages").
    Mips64,

    /// 64 bit RISC ISA ("Microprocessor without Interlocked Pipelined Stages"),
    /// but little endian.
    Mips64el,

    /// 32 bit RISC ISA (r6)
    MipsIsa32r6,

    /// 32 bit RISC ISA (r6), but little endian.
    MipsIsa32r6el,

    /// 64 bit RISC ISA (r6)
    MipsIsa64r6,

    /// 64 bit RISC ISA (r6), but little endian.
    MipsIsa64r6el,

    /// 32 bit RISC ISA.
    PowerPc,

    /// 64 bit RISC ISA.
    PowerPc64,

    /// 64 bit RISC ISA, but little endian.
    PowerPc64le,

    /// 64 bit RISC-V RISC ISA.
    RiscV64,

    /// IBM System/390
    S390,

    /// 64 bit IBM System/390
    S390X,

    /// SuperH 4 32 bit RISC ISA.
    Sh4,

    /// Sun Microsystems SPARC 32 bit RISC ISA.
    Sparc,

    /// Sun Microsystems SPARC 64 bit RISC ISA.
    Sparc64,

    /// *The* Intel 64 bit ISA.
    X86_64,

    /// Some other exotic CPU not covered otherwise.
    Other(String),
}

/// Debian architecture tuple. This is something like `x86_64-linux-gnu`.
///
/// Tuple values are documented on the
/// [Debian Wiki](https://wiki.debian.org/Multiarch/Tuples).
///
/// ```
/// use deb::architecture::multiarch::Tuple;
///
/// let tuple: Tuple = "x86_64-linux-gnu".parse().unwrap();
///
/// // prints "x86_64-linux-gnu"
/// println!("{}", tuple);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple {
    /// CPU instruction set for this platform.
    pub instruction_set: InstructionSet,

    /// Syscall API binaries targeted for this platform must use to
    /// communicate with the environment.
    pub syscall_abi: SyscallAbi,

    /// userland / library ABI running in the targeted environment.
    pub userland: String,
}

/// Error conditions which may be encountered when parsing a String
/// into a [Tuple].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TupleParseError {
    /// Returned when the string provided to [Tuple] is empty.
    Empty,

    /// Tuple is not in the form of `isa-abi-userland` or `isa-abi`,
    /// which can not be parsed into a [Tuple] correctly.
    Malformed,

    /// Error condition passed through from the [InstructionSet] parsing
    /// routine.
    BadInstructionSet(InstructionSetParseError),

    /// Error condition passed through from the [SyscallAbi] parsing
    /// routine.
    BadSyscallAbi(SyscallAbiParseError),
}

impl From<InstructionSetParseError> for TupleParseError {
    fn from(e: InstructionSetParseError) -> Self {
        Self::BadInstructionSet(e)
    }
}

impl From<SyscallAbiParseError> for TupleParseError {
    fn from(e: SyscallAbiParseError) -> Self {
        Self::BadSyscallAbi(e)
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let isa = self.instruction_set.as_str();
        let abi = self.syscall_abi.as_str();
        let userland = self.userland.as_str();

        write!(
            f,
            "{}",
            if abi == userland {
                format!("{}-{}", isa, abi)
            } else {
                format!("{}-{}-{}", isa, abi, userland)
            }
        )
    }
}

impl FromStr for Tuple {
    type Err = TupleParseError;

    fn from_str(tuple: &str) -> Result<Self, TupleParseError> {
        if tuple.is_empty() {
            return Err(TupleParseError::Empty);
        }
        let chunks: Vec<&str> = tuple.split("-").collect();

        Ok(match chunks[..] {
            [isa, abi, userland] => Tuple {
                instruction_set: isa.parse()?,
                syscall_abi: abi.parse()?,
                userland: userland.to_owned(),
            },
            [isa, abi] => Tuple {
                instruction_set: isa.parse()?,
                syscall_abi: abi.parse()?,
                userland: abi.to_owned(),
            },
            _ => return Err(TupleParseError::Malformed),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_syscall_abi_to_string() {
        assert_eq!("linux", SyscallAbi::Linux.as_str());
        assert_eq!("kfreebsd", SyscallAbi::FreeBSD.as_str());
        assert_eq!("gnu", SyscallAbi::Hurd.as_str());
        assert_eq!("uefi", SyscallAbi::Uefi.as_str());
        assert_eq!(
            "somethingelse",
            SyscallAbi::Other("somethingelse".to_owned()).as_str()
        );
    }

    #[test]
    fn check_syscall_abi_from_string() {
        assert_eq!(SyscallAbi::Linux, "linux".parse().unwrap());
        assert_eq!(SyscallAbi::FreeBSD, "kfreebsd".parse().unwrap());
        assert_eq!(SyscallAbi::Hurd, "gnu".parse().unwrap());
        assert_eq!(SyscallAbi::Uefi, "uefi".parse().unwrap());
        assert_eq!(
            SyscallAbi::Other("somethingelse".to_owned()),
            "somethingelse".parse().unwrap(),
        );
    }

    #[test]
    fn check_tuple_parse_simple() {
        let tuple: Tuple = "x86_64-linux-gnu".parse().unwrap();
        assert_eq!(
            Tuple {
                instruction_set: InstructionSet::X86_64,
                syscall_abi: SyscallAbi::Linux,
                userland: "gnu".to_owned(),
            },
            tuple
        );
    }

    #[test]
    fn check_tuple_parse_short() {
        let tuple: Tuple = "x86_64-uefi".parse().unwrap();
        assert_eq!(
            Tuple {
                instruction_set: InstructionSet::X86_64,
                syscall_abi: SyscallAbi::Uefi,
                userland: "uefi".to_owned(),
            },
            tuple
        );
    }

    #[test]
    fn check_tuple_parse_invalid() {
        assert!("".parse::<Tuple>().is_err());
        assert!("aarch64-linux-gnu-somethingelse".parse::<Tuple>().is_err());
        assert!("sparc".parse::<Tuple>().is_err());
    }
}

// vim: foldmethod=marker
