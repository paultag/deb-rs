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

use crate::version::Version;

/// Command passed to [ProcessUpload] to control the action taken
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
pub enum ProcessUploadCommand {
    /// Request that the package be rejected.
    Reject,

    /// Request that the package be accepted.
    Accept,
}

/// Migrate or block a staged upload
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProcessUpload {
    /// Action to take on the package.
    pub command: ProcessUploadCommand,

    /// Source package to run against
    pub source: String,

    /// Version of the package to run against
    pub version: Version,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::dak::{Command, CommandAction};
    use std::io::{BufReader, Cursor};

    #[test]
    fn command() {
        let command = Command::from_reader(&mut BufReader::new(Cursor::new(
            "\
Archive: ftp.upload.debian.org
Uploader: Paul Tagliamonte <paultag@debian.org>

Action: process-upload
Command: ACCEPT
Source: foo
Version: 1.0-1
",
        )))
        .unwrap();

        assert_eq!(command.header.archive, "ftp.upload.debian.org");
        assert_eq!(1, command.actions.len());

        let Some(CommandAction::ProcessUpload(process_upload)) = command.actions.get(0) else {
            panic!("bad type");
        };

        assert_eq!(process_upload.command, ProcessUploadCommand::Accept);
        assert_eq!(process_upload.source, "foo");
        assert_eq!(process_upload.version.to_string(), "1.0-1");
    }
}

// vim: foldmethod=marker
