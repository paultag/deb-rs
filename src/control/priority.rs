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

use super::def_serde_traits_for;
use std::str::FromStr;

/// Each package must have a priority value, which is set in the metadata for
/// the Debian archive and is also included in the package’s control files
/// (see Priority). This information is used to control which packages are
/// included in standard or minimal Debian installations.
///
/// Most Debian packages will have a priority of optional. Priority levels
/// other than optional are only used for packages that should be included by
/// default in a standard installation of Debian.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Priority {
    /// Packages which are necessary for the proper functioning of the system
    /// (usually, this means that dpkg functionality depends on these
    /// packages). Removing a required package may cause your system to become
    /// totally broken and you may not even be able to use dpkg to put things
    /// back, so only do so if you know what you are doing.
    ///
    /// Systems with only the required packages installed have at least enough
    /// functionality for the sysadmin to boot the system and install more
    /// software.
    Required,

    /// Important programs, including those which one would expect to find on
    /// any Unix-like system. If the expectation is that an experienced Unix
    /// person who found it missing would say "What on earth is going on,
    /// where is foo?", it must be an important package. 6 Other packages
    /// without which the system will not run well or be usable must also have
    /// priority important. This does not include Emacs, the X Window System,
    /// TeX or any other large applications. The important packages are just a
    /// bare minimum of commonly-expected and necessary tools.
    Important,

    /// These packages provide a reasonably small but not too limited
    /// character-mode system. This is what will be installed by default if
    /// the user doesn’t select anything else. It doesn’t include many large
    /// applications.
    ///
    /// Two packages that both have a priority of standard or higher must not
    /// conflict with each other.
    Standard,

    /// This is the default priority for the majority of the archive. Unless a
    /// package should be installed by default on standard Debian systems, it
    /// should have a priority of optional. Packages with a priority of optional
    /// may conflict with each other.
    Optional,

    /// This priority is deprecated. Use the optional priority instead.
    /// This priority should be treated as equivalent to optional.
    Extra,
}

/// Error conditions which may be encountered when working with a [Priority]
/// field.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PriorityParseError {
    /// Priority was empty! Can't turn that into a [Priority], now can we.
    Empty,

    /// We found an unknown string (not that the error itself is of an
    /// unknown origin -- we know very well what happened here) -- the only
    /// valid values are fairly tightly defined by Debian policy. Please be
    /// sure that the [Priority] is spelled right.
    Unknown,
}

impl FromStr for Priority {
    type Err = PriorityParseError;

    fn from_str(priority: &str) -> Result<Self, PriorityParseError> {
        Ok(match priority {
            "required" => Priority::Required,
            "important" => Priority::Important,
            "standard" => Priority::Standard,
            "optional" => Priority::Optional,
            "extra" => Priority::Extra,
            "" => return Err(PriorityParseError::Empty),
            _ => return Err(PriorityParseError::Unknown),
        })
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Priority::Required => "required",
                Priority::Important => "important",
                Priority::Standard => "standard",
                Priority::Optional => "optional",
                Priority::Extra => "extra",
            }
        )
    }
}

def_serde_traits_for!(Priority);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_loops {
        ( $name:ident, from priority $priority:expr ) => {
            #[test]
            fn $name() {
                let prio_str = $priority.to_string();
                let prio: Priority = prio_str.parse().unwrap();
                assert_eq!($priority, prio);
            }
        };

        ( $name:ident, from str $priority_str:expr ) => {
            #[test]
            fn $name() {
                let prio: Priority = $priority_str.parse().unwrap();
                let prio_str = prio.to_string();
                assert_eq!($priority_str, prio_str);
            }
        };
    }

    macro_rules! check_fails {
        ( $name:ident, $priority:expr ) => {
            #[test]
            fn $name() {
                assert!($priority.parse::<Priority>().is_err());
            }
        };
    }

    check_loops!(enum_required,  from priority Priority::Required);
    check_loops!(enum_important, from priority Priority::Important);
    check_loops!(enum_standard,  from priority Priority::Standard);
    check_loops!(enum_optional,  from priority Priority::Optional);
    check_loops!(enum_extra,     from priority Priority::Extra);

    check_loops!(str_required,   from str      "required");
    check_loops!(str_important,  from str      "important");
    check_loops!(str_standard,   from str      "standard");
    check_loops!(str_optional,   from str      "optional");
    check_loops!(str_extra,      from str      "extra");

    check_fails!(fails_empty, "");
    check_fails!(fails_bogus, "bogus");
}

// vim: foldmethod=marker
