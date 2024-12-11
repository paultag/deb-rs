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
    pest::Rule, ArchConstraints, BuildProfileRestrictionFormula, Error, VersionConstraint,
};
use crate::architecture::Architecture;
use pest::iterators::Pair;

/// A [Package] is the lowest level of [crate::dependency::Dependency]
/// relationships -- a specific package which may be used to satisfy a
/// requirement.
///
/// There are a number of constraints which limit how this [Package]
/// may be considered. Those are parsed and exported as members of the
/// [Package]. In order for a [Package] to be met, all the constraints
/// which limit the consideration of the package must be met.
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see a [Package] by parsing a
/// [crate::dependency::Dependency].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Package {
    /// Name of the package which may satisfy this particular Dependency
    /// relationship. In general, this would require the installation, but
    /// not always.
    pub name: String,

    /// [Architecture] of the package. This is not a constraint -- this is
    /// used when the package is of a specific [Architecture], which is
    /// likely not that of the host.
    ///
    /// For instance, the package `example:arm64` means that the `example`
    /// package's `.deb` files to be downloaded and installed are from the
    /// [Architecture::Arm64] Architecture, not the host's.
    ///
    /// This is seen most commonly in places where the two [Architecture]
    /// values can be run on the same ISA, such as [Architecture::Amd64] and
    /// [Architecture::I386], or [Architecture::Armhf] and
    /// [Architecture::Amd64], but it can be present in other cases too.
    pub arch: Option<Architecture>,

    /// This constraint limits the [crate::version::Version] of the package
    /// which satisfies this [Package].
    ///
    /// If the version does not match the constraint, this [Package] does
    /// not satisfy the [crate::dependency::Relation], and other [Package]
    /// values must be considered.
    pub version_constraint: Option<VersionConstraint>,

    /// This constraint limits the host [Architecture] to only consider
    /// this [Package] if the host [Architecture] matches the
    /// [ArchConstraints].
    ///
    /// If the [Architecture] does not match the constraint, this [Package]
    /// does not satisfy the [crate::dependency::Relation], and other
    /// [Package] values must be considered.
    pub arch_constraints: Option<ArchConstraints>,

    /// This constraint limits the `build_profile` to only consider
    /// this [Package] if the build build_profile configuration matches all
    /// the provided [BuildProfileRestrictionFormula].
    ///
    /// If the `build_profile` does not match the constraint, this [Package]
    /// does not satisfy the [crate::dependency::Relation], and other
    /// [Package] values must be considered.
    ///
    /// This is generally seen when bootstrapping Debian, and isn't commonly
    /// used by developers, unless their package is part of a particularly
    /// gnarly build loop, in order to make a cyclic
    /// [crate::dependency::Dependency] package relationship acyclic.
    pub build_profile_restriction_formula: Option<BuildProfileRestrictionFormula>,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)?;

        if let Some(arch) = &self.arch {
            write!(f, ":{}", arch)?;
        }

        if let Some(version_constraint) = &self.version_constraint {
            write!(
                f,
                " ({} {})",
                version_constraint.operator, version_constraint.version
            )?;
        }

        if let Some(arch_constraints) = &self.arch_constraints {
            write!(f, " [{}]", arch_constraints)?;
        }

        if let Some(bprf) = &self.build_profile_restriction_formula {
            for build_profile_constraints in &bprf.build_profile_constraints {
                write!(f, " <{}>", build_profile_constraints)?;
            }
        }

        Ok(())
    }
}

impl TryFrom<Pair<'_, Rule>> for Package {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut ret = Package {
            ..Default::default()
        };

        for constraint in token.into_inner() {
            match constraint.as_rule() {
                Rule::package_name => ret.name = constraint.as_str().to_owned(),
                Rule::arch_name => {
                    if ret.arch.is_some() {
                        return Err(Error::InvalidPackage);
                    }

                    ret.arch = Some(constraint.as_str().to_owned().parse()?)
                }
                Rule::version_constraint => {
                    if ret.version_constraint.is_some() {
                        return Err(Error::TooManyVersions);
                    }
                    ret.version_constraint = Some(constraint.try_into()?);
                }
                Rule::arch_constraints => {
                    if ret.arch_constraints.is_some() {
                        return Err(Error::TooManyArches);
                    }
                    ret.arch_constraints = Some(constraint.try_into()?);
                }
                Rule::build_profile_constraints => {
                    if ret.build_profile_restriction_formula.is_none() {
                        ret.build_profile_restriction_formula = Some(Default::default());
                    }

                    let Some(ref mut bprf) = &mut ret.build_profile_restriction_formula else {
                        unreachable!();
                    };

                    bprf.build_profile_constraints.push(constraint.try_into()?);
                }
                _ => continue,
            };
        }

        Ok(ret)
    }
}

// vim: foldmethod=marker
