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

/// Debian Maintainer ACL action(s).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct BreakTheArchive {}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use crate::control::dak::{Command, CommandAction};
    use std::io::{BufReader, Cursor};

    #[test]
    fn command() {
        let command = Command::from_reader(&mut BufReader::new(Cursor::new(
            "\
Archive: ftp.upload.debian.org
Uploader: Paul Tagliamonte <paultag@debian.org>

Action: break-the-archive
",
        )))
        .unwrap();

        assert_eq!(command.header.archive, "ftp.upload.debian.org");
        assert_eq!(1, command.actions.len());

        let Some(CommandAction::BreakTheArchive(_)) = command.actions.first() else {
            panic!("bad type");
        };
    }
}

// vim: foldmethod=marker
