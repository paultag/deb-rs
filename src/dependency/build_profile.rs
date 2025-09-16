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
    Error,
    pest::{DependencyParser, Rule},
};
use crate::build_profile::BuildProfile;
use pest::Parser;
use pest::iterators::Pair;
use std::str::FromStr;

/// A [BuildProfileConstraint] limits a [crate::dependency::Package] to only be
/// considered on a subset of all [BuildProfile] values. This can be expressed
/// via negation (for instance `!nodoc` for "Everything except
/// `nodoc`"), or providing the [BuildProfile] name (such as `build_profile1`).
#[derive(Clone, Debug, PartialEq)]
pub struct BuildProfileConstraint {
    /// True if the [BuildProfileConstraint] is inverted -- meaning, this matches
    /// any [BuildProfile] that does *not* match the provided [BuildProfile].
    pub negated: bool,

    /// [BuildProfile] that is being constrained. Depending on `negated` this
    /// may indicate the [crate::dependency::Package] that this
    /// [BuildProfileConstraint] is attached to either has explicit support or lack of
    /// support on the specified [BuildProfile].
    pub build_profile: BuildProfile,
}

impl std::fmt::Display for BuildProfileConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.negated { "!" } else { "" },
            self.build_profile
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for BuildProfileConstraint {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut negated: bool = false;
        let mut build_profile: Option<String> = None;

        for token in token.into_inner() {
            match token.as_rule() {
                Rule::not => {
                    if negated {
                        return Err(Error::InvalidBuildProfileConstraint);
                    }
                    negated = true;
                }
                Rule::build_profile_name => {
                    build_profile = Some(token.as_str().to_owned());
                }
                _ => continue,
            };
        }

        let Some(build_profile) = build_profile else {
            return Err(Error::InvalidBuildProfileConstraint);
        };

        Ok(BuildProfileConstraint {
            negated,
            build_profile: build_profile.parse()?,
        })
    }
}

/// List of [BuildProfileConstraints] between two packages
/// which limit a [crate::dependency::Package] to specific [BuildProfile]
/// values.
#[derive(Clone, Debug, PartialEq)]
pub struct BuildProfileConstraints {
    /// BuildProfilees with an expressed constraint.
    pub build_profiles: Vec<BuildProfileConstraint>,
}

impl std::fmt::Display for BuildProfileConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.build_profiles
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for BuildProfileConstraints {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut constraints = BuildProfileConstraints {
            build_profiles: vec![],
        };

        for token in token.into_inner() {
            match token.as_rule() {
                Rule::build_profile_constraint => {}
                // TODO: validation here
                _ => continue,
            };

            constraints.build_profiles.push(token.try_into()?);
        }

        Ok(constraints)
    }
}

/// List of [BuildProfileConstraints] to be satisfied. Like a
/// [crate::dependency::Dependency], every one of these
/// [BuildProfileConstraints] must be satisfied in order to consider the
/// [crate::dependency::Package].
///
/// In general you won't be parsing this directly, although it is possible
/// (for instance in a `Build-Profiles` header), in which case this can be
/// parsed directly, similar to a [crate::dependency::Dependency].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BuildProfileRestrictionFormula {
    /// List of [BuildProfileConstraints], which all must be satisfied in
    /// order to satisfy this constraint.
    pub build_profile_constraints: Vec<BuildProfileConstraints>,
}

impl From<Vec<BuildProfileConstraints>> for BuildProfileRestrictionFormula {
    fn from(constraints: Vec<BuildProfileConstraints>) -> Self {
        Self {
            build_profile_constraints: constraints,
        }
    }
}

impl std::fmt::Display for BuildProfileRestrictionFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.build_profile_constraints
                .iter()
                .map(|v| format!("<{v}>"))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for BuildProfileRestrictionFormula {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut ret = Self::default();
        for constraints in token.into_inner() {
            match constraints.as_rule() {
                Rule::build_profile_constraints => {}
                // TODO: validation here
                _ => continue,
            };
            ret.build_profile_constraints.push(constraints.try_into()?);
        }
        Ok(ret)
    }
}

impl FromStr for BuildProfileRestrictionFormula {
    type Err = Error;

    fn from_str(v: &str) -> Result<Self, Error> {
        let Some(token) =
            DependencyParser::parse(Rule::build_profile_restriction_formula, v)?.next()
        else {
            // No dependencies, empty.
            return Ok(Default::default());
        };
        token.try_into()
    }
}

impl BuildProfileConstraint {
    /// Return true if the provided [BuildProfile] meets the requirements
    /// in the [BuildProfileConstraint].
    pub fn matches(&self, build_profile: &BuildProfile) -> bool {
        let matches = self.build_profile == *build_profile;

        if self.negated { !matches } else { matches }
    }
}

impl BuildProfileConstraints {
    /// Return true if ALL of the provided [BuildProfile]s meets ANY of
    /// the requirements in this set of [BuildProfileConstraints].
    pub fn matches(&self, build_profiles: &[BuildProfile]) -> bool {
        self.build_profiles.iter().any(|bpc| {
            // for each build profile constraint, we need to check
            // to see if all of the provided profiles match this
            build_profiles.iter().all(|bp| bpc.matches(bp))
        })
    }
}

impl BuildProfileRestrictionFormula {
    /// Return true if ALL the provided [BuildProfile]s meet ALL of the
    /// [BuildProfileConstraints].
    pub fn matches(&self, build_profile: &[BuildProfile]) -> bool {
        self.build_profile_constraints
            .iter()
            .all(|bpc| bpc.matches(build_profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_profiles_syntax() {
        let bprf: BuildProfileRestrictionFormula = "<!cross> <!nocheck>".parse().unwrap();
        assert_eq!(2, bprf.build_profile_constraints.len());
        assert_eq!(1, bprf.build_profile_constraints[0].build_profiles.len());
        assert_eq!(1, bprf.build_profile_constraints[1].build_profiles.len());
    }

    #[test]
    fn test_build_profile_matches() {
        let bprf: BuildProfileRestrictionFormula = "<!cross> <!nocheck>".parse().unwrap();

        let cross: BuildProfile = "cross".parse().unwrap();
        let nocheck: BuildProfile = "nocheck".parse().unwrap();
        let nodoc: BuildProfile = "nodoc".parse().unwrap();

        assert!(!bprf.matches(&[cross]));
        assert!(!bprf.matches(&[nocheck]));
        assert!(bprf.matches(&[nodoc]));
    }

    #[test]
    fn test_build_profile_muiltiple() {
        let bprf: BuildProfileRestrictionFormula = "<!cross !nodoc>".parse().unwrap();

        assert!(!bprf.matches(&[BuildProfile::Cross, BuildProfile::NoDoc]));
        assert!(bprf.matches(&[BuildProfile::NoDoc]));
        assert!(bprf.matches(&[BuildProfile::Cross]));

        let bprf: BuildProfileRestrictionFormula = "<!cross !nodoc> <!nogolang>".parse().unwrap();

        assert!(!bprf.matches(&[BuildProfile::Cross, BuildProfile::NoDoc]));
        assert!(bprf.matches(&[BuildProfile::NoDoc]));
        assert!(bprf.matches(&[BuildProfile::Cross]));

        assert!(!bprf.matches(&[BuildProfile::NoGolang]));
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::BuildProfileRestrictionFormula;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

    impl Serialize for BuildProfileRestrictionFormula {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            String::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de> Deserialize<'de> for BuildProfileRestrictionFormula {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let s = String::deserialize(d)?;
            s.parse().map_err(|e| D::Error::custom(format!("{e:?}")))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::control;
        use std::io::{BufReader, Cursor};

        #[test]
        fn serde_build_profile_restriction_formula() {
            #[derive(Clone, Debug, PartialEq, Deserialize)]
            struct Test {
                #[serde(rename = "Build-Profiles")]
                build_profiles: BuildProfileRestrictionFormula,
            }

            let test: Test = control::de::from_reader(&mut BufReader::new(Cursor::new(
                "\
Build-Profiles: <foo> <bar>
",
            )))
            .unwrap();

            assert_eq!(test.build_profiles.build_profile_constraints.len(), 2);
        }
    }
}

// vim: foldmethod=marker
