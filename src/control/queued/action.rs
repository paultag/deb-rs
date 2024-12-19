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

/// Individual command to be run in the upload queue.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// Request to cancel the upload. This should be a path to
    /// a `.changes` file.
    Cancel(String),

    /// Request that an upload be moved to a specific day offset
    /// in the DELAYED queue.
    Reschedule(String),

    /// Request that an upload be removed.
    Rm(String),

    /// Unknown and unparsed command.
    Unknown(String),
}

crate::control::macros::def_serde_traits_for!(Action);

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Cancel(path) => {
                    format!("cancel {}", path)
                }
                Self::Reschedule(arg) => {
                    format!("reschedule {}", arg)
                }
                Self::Rm(args) => {
                    format!("rm {}", args)
                }
                Self::Unknown(unk) => {
                    unk.clone()
                }
            }
        )
    }
}

impl FromStr for Action {
    type Err = std::convert::Infallible;

    fn from_str(action: &str) -> Result<Self, Self::Err> {
        let Ok::<[&str; 2], _>([command, argument]) =
            action.splitn(2, ' ').collect::<Vec<_>>().try_into()
        else {
            return Ok(Self::Unknown(action.to_owned()));
        };

        Ok(match command {
            "cancel" => Self::Cancel(argument.to_owned()),
            "rm" => Self::Rm(argument.to_owned()),
            "reschedule" => Self::Reschedule(argument.to_owned()),
            _ => Self::Unknown(action.to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel() {
        let action: Action = "cancel foo.changes".parse().unwrap();
        let Action::Cancel(changes) = action else {
            panic!("Not a cancel");
        };
        assert_eq!("foo.changes", changes);
    }

    #[test]
    fn test_reschedule() {
        let action: Action = "reschedule foo.changes 1-day".parse().unwrap();
        let Action::Reschedule(arg) = action else {
            panic!("Not a reschedule");
        };
        assert_eq!("foo.changes 1-day", arg);
    }

    #[test]
    fn test_rm() {
        let action: Action = "rm --searchdirs foo.changes".parse().unwrap();
        let Action::Rm(arg) = action else {
            panic!("Not a rm");
        };
        assert_eq!("--searchdirs foo.changes", arg);
    }

    #[test]
    fn test_unknown() {
        let action: Action = "unknown something.changes".parse().unwrap();
        let Action::Unknown(args) = action else {
            panic!("Not a unknown");
        };
        assert_eq!("unknown something.changes", args);
    }
}

// vim: foldmethod=marker
