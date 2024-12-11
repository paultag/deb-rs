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

//! The `dependency` module contains support for parsing Debian
//! dependency relationships.
//!
//! This will parse fully populated Dependency relationships, as seen in
//! `.deb` control files.
//!
//!
//! # Note on `dpkg-substvars`
//!
//! This module will *not* parse [Dependency] values that contain dpkg
//! substvars. While it may, at first, seem fairly straight forward,
//! and is likely needed at some level for doing things like `wrap-and-sort`,
//! `dpkg-substvars` are a stringwise-concept, and can occur anywhere in the
//! [Dependency] string.
//!
//! For instance `foo, ${bar:Depends}` is commonly seen in control files,
//! so it may appear to be possible to store the substvar information at
//! the [Package] level, however, it's also common to see something
//! like `baz (= ${my:Version})` in, say, libraries.
//!
//! This is a bit of a gotcha, since a lot of development control files
//! contain `dpkg-substvars`, which will throw a parse error as of right
//! now. There will likely have to be multiple ways in which a Dependency
//! can be parsed and understood, but for the initial version of this module,
//! the [Dependency] must be free of `dpkg-substvar`s.
//!
//! ```
//! use deb::dependency::Dependency;
//!
//! let dep: Dependency = "package-1 <stage1>, option (= 1.0) | option-2 [arm64]".parse().unwrap();
//! ```
//!
//! # Overview of the [Dependency] model
//!
//! A [Dependency] is made up of a number of [Relation]s. All [Relation]s
//! must be satisfied. A [Relation] is made up of a number of [Package]
//! structs. Any [Package] being satisfied will satisfy the [Relation].
//! A [Package] has a number of constraints on it (such as a
//! [VersionConstraint], [ArchConstraint] or a [BuildProfileRestrictionFormula]), which
//! dictate when it can be considered.
//!
//! These terms are unique to this crate.
//!
//! # Feature `serde`
//!
//! This feature will enable derives or explicit implementations of
//! [serde::Deserialize] and [serde::Serialize] for types in this module.

mod architecture;
mod build_profile;
#[allow(clippy::module_inception)]
mod dependency;
mod package;
mod pest;
mod relation;
mod tests;
mod version;

pub use architecture::{ArchConstraint, ArchConstraints};
pub use build_profile::{
    BuildProfileConstraint, BuildProfileConstraints, BuildProfileRestrictionFormula,
};
pub use dependency::{Dependency, Error};
pub use package::Package;
pub use relation::Relation;
pub use version::{VersionConstraint, VersionOperator};

// vim: foldmethod=marker
