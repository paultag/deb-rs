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

use super::DscParseError;
use crate::control::{def_serde_traits_for, Priority};
use std::str::FromStr;

/// [PackageList] describes one binary package, by listing its name, type,
/// section and priority separated by spaces.
///
/// The package is the binary package name.
///
/// The package-type is the binary package type, usually deb, another common
/// value is udeb.
///
/// The section and priority match the binary package fields of the same name.
///
/// The key-value-list is a space separated key=value list, and the currently
/// known optional keys are `arch`, `profile`, `protected`, `essential`. Adding
/// support for this is a TODO.
#[derive(Clone, Debug, PartialEq)]
pub struct PackageList {
    /// Name of the binary package that may be produced by this source package.
    pub name: String,

    /// Indicate the type of package: deb for binary packages and udeb for
    /// micro binary packages. Other types not defined here may be indicated.
    pub binary_type: String,

    /// Section of the archive to target.
    pub section: String,

    /// Priority of the package.
    pub priority: Priority,
}

impl std::fmt::Display for PackageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} {} {} {}",
            self.name, self.binary_type, self.section, self.priority
        )
    }
}

impl FromStr for PackageList {
    type Err = DscParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [name, binary_type, section, priority] = s
            .split(" ")
            .take(4)
            .map(|v| v.to_owned())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| DscParseError::Malformed)?;

        Ok(Self {
            name,
            binary_type,
            section,
            priority: priority.parse().map_err(DscParseError::InvalidPriority)?,
        })
    }
}

def_serde_traits_for!(PackageList);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_parses {
        ($name:ident, $checksum:expr ) => {
            #[test]
            fn $name() {
                assert!($checksum.parse::<PackageList>().is_ok());
            }
        };
    }

    check_parses!(check_short, "ocaml-doc deb non-free/doc optional");
    check_parses!(
        check_with_arch,
        "ocaml-doc deb non-free/doc optional arch=all"
    );
}

// vim: foldmethod=marker
