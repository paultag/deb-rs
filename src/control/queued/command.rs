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

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::control::de;

use super::Action;

/// Command from a Debian Developer uploaded to
/// queued to request a change to the upload queue,
/// such as cancelling a delayed upload.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Command {
    /// Name of the individual requesting the change. Reply emails
    /// from queued will go here.
    pub uploader: String,

    /// Individual actions to take in the queue.
    pub commands: Vec<Action>,
}

/// Error conditions which may be encountered when working with a
/// [Command]
#[derive(Debug)]
#[cfg_attr(not(feature = "serde"), allow(missing_copy_implementations))]
pub enum CommandError {
    /// Underlying [de::Error] serde decoding error.
    #[cfg(feature = "serde")]
    De(de::Error),
}

#[cfg(feature = "serde")]
impl From<de::Error> for CommandError {
    fn from(dee: de::Error) -> Self {
        Self::De(dee)
    }
}

crate::errors::error_enum!(CommandError);

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn command_everything() {
        let command: Command = de::from_str(
            "\
Uploader: Paul Tagliamonte <paultag@debian.org>
Commands:
 rm --searchdir foo
 reschedule foo.changes 2-day
 cancel bar.changes
 unknown unknown
",
        )
        .unwrap();

        assert_eq!(4, command.commands.len());
        assert_eq!(
            &[
                Action::Rm("--searchdir foo".to_owned()),
                Action::Reschedule("foo.changes 2-day".to_owned()),
                Action::Cancel("bar.changes".to_owned()),
                Action::Unknown("unknown unknown".to_owned())
            ],
            command.commands.as_slice()
        );
    }
}

// vim: foldmethod=marker
