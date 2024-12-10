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

use super::{pest::Rule, Error, Possibility};
use pest::iterators::Pair;

/// A [Relation] is a set of [Possibility] values, any one of which will
/// satisfy the [Relation] requirement.
///
/// This is different from a [crate::dependency::Dependency], since in a
/// Dependency, all [Relation] must be satisfied to meet the requirement
/// (an AND relationship), whereas in a [Relation], any [Possibility] value
/// satisfies the [Relation] (an OR relationship).
///
/// In general, you're unlikely to be parsing these directly, instead
/// you're likely going to see a [Relation] by parsing a
/// [crate::dependency::Dependency].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Relation {
    /// Set of [Possibility] values, any one of which satisfies the
    /// the [Relation].
    pub possibilities: Vec<Possibility>,
}

impl std::fmt::Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.possibilities
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

impl TryFrom<Pair<'_, Rule>> for Relation {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut ret = Relation {
            possibilities: vec![],
        };
        for possibility in token.into_inner() {
            match possibility.as_rule() {
                Rule::possibility => {}
                // TODO: validation here
                _ => continue,
            };
            ret.possibilities.push(possibility.try_into()?);
        }

        Ok(ret)
    }
}

// vim: foldmethod=marker
