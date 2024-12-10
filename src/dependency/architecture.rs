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
use crate::architecture::Architecture;
use pest::iterators::Pair;

/// An [ArchConstraint] limits a [crate::dependency::Possibility] to only be
/// considered on a subset of all [Architecture] values. This can be expressed
/// via negation (for instance `!amd64` for "Everything except
/// [Architecture::Amd64]"), or providing the [Architecture] name
/// (such as `arm64`).
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see an [ArchConstraint] by parsing a
/// [crate::dependency::Dependency], and getting the [ArchConstraint] off the
/// [crate::dependency::Possibility].
#[derive(Clone, Debug, PartialEq)]
pub struct ArchConstraint {
    /// True if the [ArchConstraint] is inverted -- meaning, this matches
    /// any [Architecture] that does *not* match the provided [Architecture].
    pub negated: bool,

    /// [Architecture] that is being constrained. Depending on `negated` this
    /// may indicate the [crate::dependency::Possibility] that this
    /// [ArchConstraint] is attached to either has explicit support or lack of
    /// support on the specified [Architecture].
    pub arch: Architecture,
}

impl std::fmt::Display for ArchConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.negated { "!" } else { "" }, self.arch)
    }
}

impl TryFrom<Pair<'_, Rule>> for ArchConstraint {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut negated: bool = false;
        let mut arch: Option<String> = None;

        for token in token.into_inner() {
            match token.as_rule() {
                Rule::not => {
                    if negated {
                        return Err(Error::InvalidArchConstraint);
                    }
                    negated = true;
                }
                Rule::arch_name => {
                    arch = Some(token.as_str().to_owned());
                }
                _ => continue,
            };
        }

        let Some(arch) = arch else {
            return Err(Error::InvalidArchConstraint);
        };

        Ok(ArchConstraint {
            negated,
            arch: arch.parse()?,
        })
    }
}

/// List of [ArchConstraint] values which limit a
/// [crate::dependency::Possibility] to specific [Architecture] values.
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see an [ArchConstraints] value by parsing a
/// [crate::dependency::Dependency], and getting the [ArchConstraints] off the
/// [crate::dependency::Possibility].
#[derive(Clone, Debug, PartialEq)]
pub struct ArchConstraints {
    /// List of [ArchConstraint] values. These are treated as an "AND",
    /// for the [crate::dependency::Possibility] this struct is attached to,
    /// all [ArchConstraint] values must be satisfied by an [Architecture]
    /// to be considered valid
    /// for it.
    pub arches: Vec<ArchConstraint>,
}

impl std::fmt::Display for ArchConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.arches
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for ArchConstraints {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut constraints = ArchConstraints { arches: vec![] };
        for token in token.into_inner() {
            match token.as_rule() {
                Rule::arch_constraint => {}
                // TODO: validation here
                _ => continue,
            };
            constraints.arches.push(token.try_into()?);
        }
        Ok(constraints)
    }
}

// vim: foldmethod=marker
