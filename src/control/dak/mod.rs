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

//! Rust types to handle Deserialization of
//! [dak](https://ftp-master.debian.org/#dak)-specific `.dak-commands`
//! files used by Debian Developers to control the archive.
//!
//! # Dak's `.dak-commands` control types
//!
//! A generic dak command can be decoded into a [Command] using something like
//! [Command::from_reader] or [Command::from_reader_async]. Each command file
//! starts with a [CommandHeader], followed by any number of [CommandAction]s.
//!
//! | What                   | Action Name           | Struct                 |
//! | ---------------------- | --------------------- | ---------------------- |
//! | Manage DM Permissions  | `dm`                  | [Dm]                   |
//! | Break the Archive      | `break-the-archive`   | [BreakTheArchive]      |
//! | Process an Upload      | `process-upload`      | [ProcessUpload]        |

mod break_the_archive;
mod dm;
mod process_upload;

pub use break_the_archive::BreakTheArchive;
pub use dm::Dm;
pub use process_upload::ProcessUpload;

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use crate::control::de;

/// Command from a Debian Developer uploaded to
/// [dak](https://ftp-master.debian.org/#dak) to request a change to the
/// archive, such as granting Debian Maintainer rights via [CommandAction::Dm].
#[derive(Clone, Debug)]
pub struct Command {
    /// Dak command header, this specifies what archive this was uploaded to
    /// in order to prevent reuse between archives.
    pub header: CommandHeader,

    /// Individual change requests.
    pub actions: Vec<CommandAction>,
}

/// Error conditions which may be encountered when working with a
/// [Command], [CommandHeader] or a [CommandAction].
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

/// Possible [dak](https://ftp-master.debian.org/#dak) commands which are
/// uploaded by Debian Developers in order to control the archive.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "serde", serde(tag = "Action"))]
pub enum CommandAction {
    /// Add or remove Debian Maintainer (DM) rights.
    Dm(Dm),

    /// Break the archive
    #[cfg_attr(feature = "serde", serde(rename = "break-the-archive"))]
    BreakTheArchive(BreakTheArchive),

    /// Migrate or block a package.
    #[cfg_attr(feature = "serde", serde(rename = "process-upload"))]
    ProcessUpload(ProcessUpload),
}

/// First paragraph of a Dak commands file (`.dak-commands`) in order
/// to prevent reuse between archives who have a shared developer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct CommandHeader {
    /// Archive that this Command is intended for. Usually a FQDN.
    pub archive: String,

    /// Optional identity of the individual uploading the command.
    pub uploader: Option<String>,
}

#[cfg(feature = "serde")]
mod _serde {
    #![cfg_attr(docsrs, doc(cfg(feature = "serde")))]

    use super::*;
    use std::io::{BufReader, Read};

    impl Command {
        /// Parse a [Command] from a [std::io::Read] traited object.
        pub fn from_reader<ReadT>(read: &mut BufReader<ReadT>) -> Result<Command, CommandError>
        where
            ReadT: Read,
        {
            let header: CommandHeader = de::from_reader(read)?;
            Ok(Command {
                header,
                actions: de::from_reader_iter(read).collect::<Result<Vec<_>, _>>()?,
            })
        }
    }

    #[cfg(feature = "tokio")]
    mod _tokio {
        #![cfg_attr(docsrs, doc(cfg(feature = "tokio")))]

        use super::*;
        use crate::control::de;
        use tokio::io::{AsyncRead, BufReader};

        impl Command {
            /// Parse a [Command] from a [tokio::io::AsyncRead] traited object.
            pub async fn from_reader_async<ReadT>(
                read: &mut BufReader<ReadT>,
            ) -> Result<Command, CommandError>
            where
                ReadT: AsyncRead,
                ReadT: Unpin,
            {
                let header: CommandHeader = de::from_reader_async(read).await?;
                let mut actions = vec![];
                while let Some(action) = de::from_reader_async_iter(read).next().await {
                    let action = action?;
                    actions.push(action);
                }
                Ok(Command { header, actions })
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use std::io::Cursor;

            #[tokio::test]
            async fn command() {
                let command = Command::from_reader_async(&mut BufReader::new(Cursor::new(
                    "\
Archive: ftp.upload.debian.org
Uploader: Paul Tagliamonte <paultag@debian.org>

Action: dm
Fingerprint: 1234567890ABCDEF1234567890ABCDEF
Allow: one-package another-package
Deny: yet-another-package

Action: dm
Fingerprint: 1234567890ABCDEF1234567890ABCDEF
Allow: one
Deny: two
",
                )))
                .await
                .unwrap();

                assert_eq!(command.header.archive, "ftp.upload.debian.org");
                assert_eq!(2, command.actions.len());
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::control::de;
        use std::io::Cursor;

        #[test]
        fn command_dm() {
            let command: CommandAction = de::from_str(
                "\
Action: dm
Fingerprint: 1234567890ABCDEF1234567890ABCDEF
Allow: one-package another-package
Deny: yet-another-package
",
            )
            .unwrap();

            let CommandAction::Dm(dm) = command else {
                panic!("not a dm command");
            };

            assert_eq!("1234567890ABCDEF1234567890ABCDEF", &*dm.fingerprint);
            assert_eq!(&["one-package", "another-package"], &*dm.allow.unwrap());
            assert_eq!(&["yet-another-package"], &*dm.deny.unwrap());
        }

        #[test]
        fn command_header() {
            let header: CommandHeader = de::from_str(
                "\
Archive: ftp.upload.debian.org
Uploader: Paul Tagliamonte <paultag@debian.org>
",
            )
            .unwrap();

            assert_eq!("ftp.upload.debian.org", &header.archive);
            assert_eq!(
                "Paul Tagliamonte <paultag@debian.org>",
                header.uploader.unwrap()
            );
        }

        #[test]
        fn command() {
            let command = Command::from_reader(&mut BufReader::new(Cursor::new(
                "\
Archive: ftp.upload.debian.org
Uploader: Paul Tagliamonte <paultag@debian.org>

Action: dm
Fingerprint: 1234567890ABCDEF1234567890ABCDEF
Allow: one-package another-package
Deny: yet-another-package

Action: dm
Fingerprint: 1234567890ABCDEF1234567890ABCDEF
Allow: one
Deny: two
",
            )))
            .unwrap();

            assert_eq!(command.header.archive, "ftp.upload.debian.org");
            assert_eq!(2, command.actions.len());
        }
    }
}

// vim: foldmethod=marker
