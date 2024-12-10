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

use std::str::FromStr;

/// A [BuildProfile] is used to assist with breaking complex package
/// relationships, such as is the case with bootstrapping the Debian distribution,
/// or cross-building.
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see a [BuildProfile] by parsing a
/// [crate::dependency::Dependency], or a
/// [crate::dependency::BuildProfileRestrictionFormula], as seen in
/// `Build-Profile` and friends.
///
/// Current package build profiles can be found on the Debian
/// [wiki](https://wiki.debian.org/BuildProfileSpec). A non-exhaustive set
/// of stages is provided below.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum BuildProfile {
    /// Can be used to add extra dependencies for cross-compilation. Usually
    /// used when a package would build depend on it self. If the set of binary
    /// packages built changes, then it should be combined with another profile.
    Cross,

    /// Extension namespace. Can be used whenever the maintainer of
    /// `$sourcepackage` agrees to the use. `$anything` must match the
    /// following regex `[a-z0-9-]+`.
    Pkg(String, String),

    /// Deprecated. Use a descriptive profile below or from the extension
    /// namespace instead. Must reduce `Build-Depends`. Must not be used
    /// outside of the very early cross-compiler bootstrap phase (`gcc`,
    /// `glibc`, `linux`)
    Stage1,

    /// Deprecated. Use a descriptive profile below or from the extension
    /// namespace instead. Should not be necessary for packages other than
    /// `glibc` or `gcc`.
    Stage2,

    /// No multilib packages should be built.
    NoBiarch,

    /// No test suite should be run, and build dependencies used only for that
    /// purpose should be ignored. Builds that set this profile must also add
    /// `nocheck` to `DEB_BUILD_OPTIONS`.
    NoCheck,

    /// Disable CIL (Common Intermediate Language) bindings, as used by the
    /// Common Language Infrastructure, Mono, .NET and C#
    NoCil,

    /// No documentation should be included or built into packages. Builds that
    /// set this profile must also add `nodoc` to `DEB_BUILD_OPTIONS`.
    NoDoc,

    /// Disable GObject-Introspection bindings (`.gir`, `.typelib`).
    NoGir,

    /// Disable bindings for the Go programming language
    NoGolang,

    /// Disable binary packages consisting entirely of automated tests, manual
    /// tests, example/demo programs and test tools.
    NoInsttest,

    /// Disable Java bindings.
    NoJava,

    /// Disable Perl bindings.
    NoPerl,

    /// Disable Perl bindings (all versions).
    NoPython,

    /// Disable Ruby bindings (all versions).
    NoRuby,

    /// Disable Lua bindings (all versions).
    NoLua,

    /// Disable Guile bindings.
    NoGuile,

    /// Disable OCaml bindings.
    NoOcaml,

    /// Disable WASM components.
    NoWasm,

    /// Disable Windows components (can include anything using the Windows ABI,
    /// all versions)
    NoWindows,

    /// Inhibit building udebs
    NoUdeb,

    /// Use Rust dependencies from upstream. Certain selected packages only.
    UpstreamCargo,

    /// Other Build Profile, not currently understood.
    Unknown(String),
}

#[allow(unused_macros)]
macro_rules! build_profile_table_tests {
    ( $( ( $id:ident, $name:expr, $bp:path ) ),* ) => {
$(
    #[test]
    fn $id() {
        let bp: BuildProfile = $name.parse().unwrap();
        assert_eq!(&bp.to_string(), $name);
    }
)*
    };
}

macro_rules! build_profile_table_as_str {
    ( $( ( $id:ident, $name:expr, $bp:path ) ),* ) => {
impl std::fmt::Display for BuildProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            $( $bp => $name.to_owned(), )*
            Self::Unknown(v) => v.to_owned(),
            Self::Pkg(source, anything) => format!("pkg.{}.{}", source, anything),
        })
    }
}


    }
}

macro_rules! build_profile_table_from_str {
    ( $( ( $id:ident, $name:expr, $bp:path ) ),* ) => {
impl FromStr for BuildProfile {
    type Err = Error;

    fn from_str(bp: &str) -> Result<Self, Error> {
        if bp.starts_with("pkg.") {
            match bp.splitn(3, '.').collect::<Vec<_>>()[..] {
                [_, package, anything] => {
                    return Ok(Self::Pkg(package.to_owned(), anything.to_owned()))
                }
                _ => {
                    return Err(Error::InvalidPkgFormat)
                }
            }
        }

        Ok(match bp {
            $( $name => $bp, )*
            _ => BuildProfile::Unknown(bp.to_owned()),
        })
    }
}
    }
}

macro_rules! build_profile_table {
    ( $( ( $id:ident, $name:expr, $bp:path ) ),* ) => {
        build_profile_table_from_str!($( ($id, $name, $bp) ),*);
        build_profile_table_as_str!($( ($id, $name, $bp) ),*);

        #[cfg(test)]
        mod arch_tests {
            use super::*;
            build_profile_table_tests!($( ($id, $name, $bp) ),*);
        }
    };
}

build_profile_table!(
    (cross, "cross", BuildProfile::Cross),
    (stage1, "stage1", BuildProfile::Stage1),
    (stage2, "stage2", BuildProfile::Stage2),
    (nobiarch, "nobiarch", BuildProfile::NoBiarch),
    (nocheck, "nocheck", BuildProfile::NoCheck),
    (nocil, "nocil", BuildProfile::NoCil),
    (nodoc, "nodoc", BuildProfile::NoDoc),
    (nogir, "nogir", BuildProfile::NoGir),
    (nogolang, "nogolang", BuildProfile::NoGolang),
    (noinsttest, "noinsttest", BuildProfile::NoInsttest),
    (nojava, "nojava", BuildProfile::NoJava),
    (noperl, "noperl", BuildProfile::NoPerl),
    (nopython, "nopython", BuildProfile::NoPython),
    (noruby, "noruby", BuildProfile::NoRuby),
    (nolua, "nolua", BuildProfile::NoLua),
    (noguile, "noguile", BuildProfile::NoGuile),
    (noocaml, "noocaml", BuildProfile::NoOcaml),
    (nowasm, "nowasm", BuildProfile::NoWasm),
    (nowindows, "nowindows", BuildProfile::NoWindows),
    (noudeb, "noudeb", BuildProfile::NoUdeb),
    (
        upstream_cargo,
        "upstream-cargo",
        BuildProfile::UpstreamCargo
    )
);

/// Error conditions which may be encountered when parsing a String
/// into a [BuildProfile].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// Returned if the format of the `pkg.source.anything` [BuildProfile]
    /// was malformed in some way.
    InvalidPkgFormat,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pkg() {
        let bp: BuildProfile = "pkg.foo.bar".parse().unwrap();
        assert_eq!(BuildProfile::Pkg("foo".to_owned(), "bar".to_owned()), bp);
    }

    #[test]
    fn test_unknown() {
        let bp: BuildProfile = "hello".parse().unwrap();
        assert_eq!(BuildProfile::Unknown("hello".to_owned()), bp);
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::BuildProfile;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for BuildProfile {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for BuildProfile {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{:?}", e)))
        }
    }
}

// vim: foldmethod=marker
