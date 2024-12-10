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

use super::{pest::Rule, Error};
use crate::version::Version;
use pest::iterators::Pair;

/// Version constraint operator, used to limit the way the [Version] number is
/// compared to a package's [Version].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VersionOperator {
    /// Equal operator (`=`), which indicates an exact match in version
    /// number.
    Equal,

    /// GreaterThan operator (`>>`), which indicates the version must be strictly
    /// greater than the indicated Version.
    GreaterThan,

    /// LessThan operator (`<<`), which indicates the version must be strictly
    /// smaller than the indicated Version.
    LessThan,

    /// GreaterThanOrEqual operator (`>=`) which indicates the version must be
    /// greater than or equal to, the indicated version.
    GreaterThanOrEqual,

    /// LessThanOrEqual operator (`<=`) which indicates the version must be
    /// less than or equal to, the indicated version.
    LessThanOrEqual,
}

impl VersionOperator {
    /// Return the [VersionOperator] as a borrowed string ref.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Equal => "=",
            Self::GreaterThan => ">>",
            Self::LessThan => "<<",
            Self::GreaterThanOrEqual => ">=",
            Self::LessThanOrEqual => "<=",
        }
    }
}

impl std::fmt::Display for VersionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A [VersionConstraint] limits a [crate::dependency::Possibility] to only be
/// considered on a subset of all [Version] values. The [VersionOperator]
/// defines how the [Version]s are compared.
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see a [VersionOperator] by parsing a
/// [crate::dependency::Dependency].
#[derive(Clone, Debug, PartialEq)]
pub struct VersionConstraint {
    /// Comparison to use when evaluating if a [crate::dependency::Possibility]
    /// satisfies this constraint.
    pub operator: VersionOperator,

    /// Specific [Version] to compare a package against.
    pub version: Version,
}

impl std::fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.operator, self.version)
    }
}

impl TryFrom<Pair<'_, Rule>> for VersionConstraint {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut operator: Option<VersionOperator> = None;
        let mut version: Option<Version> = None;

        for token in token.into_inner() {
            match token.as_rule() {
                Rule::version_operator => {
                    operator = Some(match token.as_str() {
                        "==" => VersionOperator::Equal,
                        "=" => VersionOperator::Equal,
                        "<<" => VersionOperator::LessThan,
                        ">>" => VersionOperator::GreaterThan,
                        ">=" => VersionOperator::GreaterThanOrEqual,
                        "<=" => VersionOperator::LessThanOrEqual,
                        _ => {
                            unreachable!();
                        }
                    });
                }
                Rule::version => {
                    version = Some(token.as_str().parse()?);
                }
                // TODO: validation here
                _ => continue,
            };
        }

        let Some(operator) = operator else {
            return Err(Error::InvalidVersionConstraint);
        };

        let Some(version) = version else {
            return Err(Error::InvalidVersionConstraint);
        };

        Ok(VersionConstraint { operator, version })
    }
}

// vim: foldmethod=marker
