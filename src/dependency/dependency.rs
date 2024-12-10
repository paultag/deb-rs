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

use super::{
    pest::{DependencyParser, Rule},
    Relation,
};
use crate::{architecture, build_profile, version};
use pest::{error::Error as PestError, iterators::Pair, Parser};
use std::str::FromStr;

/// A [Dependency] is a set of constraints which must be met in order to
/// be satisfied. These are seen throughout Debian's infrastructure and
/// tools, in places like `debian/control` files, Archive index files,
/// or in `apt` output.
///
/// Each [Dependency] is comprised of a set of [Relation]s, which must all
/// be satisifed in order for the [Dependency] to be satisfied. Each [Relation]
/// is effectively an `AND` operation.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Dependency {
    /// Set of [Relation] values wich must *all* be satisfied in order for
    /// the Dependency to be satisfied.
    pub relations: Vec<Relation>,
}

/// Error conditions which may be encountered when parsing a String into a
/// [Dependency].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error parsing the Dependency line. This is currently an exported
    /// Pest error, but that _will_ change in the future. This is only
    /// like this for testing, and will be removed.
    ///
    /// TODO: remove this
    Parse((String, pest::error::InputLocation)),

    /// [crate::dependency::Possibility] String is malformed in such a way that
    /// can not be correctly parsed.
    InvalidPossibility,

    /// [crate::version::Version] String is malformed in some way that can not
    /// be correctly parsed.
    InvalidVersion(version::Error),

    /// [crate::dependency::VersionConstraint] String is malformed in some way
    /// that can not be correctly parsed.
    InvalidVersionConstraint,

    /// [crate::architecture::Architecture] String is malformed in some way
    /// that can not be correctly parsed.
    InvalidArch(architecture::Error),

    /// [crate::build_profile::BuildProfile] String is malformed in some way
    /// that could not be correctly parsed.
    InvalidBuildProfile(build_profile::Error),

    /// The [crate::dependency::ArchConstraint] String is malformed in some
    /// way that can not be correctly parsed.
    InvalidArchConstraint,

    /// The [crate::dependency::BuildProfileConstraint] String is malformed in some
    /// way that can not be correctly parsed.
    InvalidBuildProfileConstraint,

    /// Only one [crate::dependency::VersionConstraint] may be specified for a
    /// given [crate::dependency::Possibility]. This error will be returned if
    /// multiple [crate::dependency::VersionConstraint] are provided.
    TooManyVersions,

    /// Only one set of [crate::dependency::ArchConstraints] may be specified
    /// for a given [crate::dependency::Possibility]. This error will be
    /// returned if multiple [crate::dependency::ArchConstraints] are
    /// provided.
    TooManyArches,
}

impl From<architecture::Error> for Error {
    fn from(err: architecture::Error) -> Self {
        Error::InvalidArch(err)
    }
}

impl From<version::Error> for Error {
    fn from(err: version::Error) -> Self {
        Error::InvalidVersion(err)
    }
}

impl From<build_profile::Error> for Error {
    fn from(err: build_profile::Error) -> Self {
        Error::InvalidBuildProfile(err)
    }
}

impl From<PestError<Rule>> for Error {
    fn from(err: PestError<Rule>) -> Self {
        Error::Parse((err.variant.message().into(), err.location))
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.relations
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for Dependency {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut ret = Dependency { relations: vec![] };
        for relation in token.into_inner() {
            match relation.as_rule() {
                Rule::relation => {}
                // TODO: validation here
                _ => continue,
            };
            ret.relations.push(relation.try_into()?);
        }

        Ok(ret)
    }
}

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(v: &str) -> Result<Self, Error> {
        let Some(token) = DependencyParser::parse(Rule::dependency, v)?.next() else {
            // No dependencies, empty.
            return Ok(Dependency { relations: vec![] });
        };

        token.try_into()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Dependency;
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Dependency {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for Dependency {
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
        fn serde_dependency() {
            #[derive(Clone, Debug, PartialEq, Deserialize)]
            struct Test {
                #[serde(rename = "Deps")]
                deps: Dependency,
            }

            let test: Test = control::de::from_reader(&mut BufReader::new(Cursor::new(
                "\
Deps: foo, bar | baz
",
            )))
            .unwrap();

            assert_eq!(test.deps.relations.len(), 2);
        }
    }
}

// vim: foldmethod=marker
