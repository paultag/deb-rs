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

//! Helpers using the [sequoia_openpgp] crate to handle OpenPGP file signature
//! verification without using the really error prone API directly.

use sequoia_openpgp::{
    cert::CertParser,
    parse::{
        stream::{MessageLayer, MessageStructure, VerificationHelper, VerifierBuilder},
        Parse,
    },
    policy::StandardPolicy,
    Cert, Fingerprint, KeyHandle, Result as SequoiaResult,
};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

/// Builder-pattern to create an OpenPgp validator, to decode control
/// files signed by a key in the keyring(s).
#[derive(Clone, Default)]
pub struct OpenPgpValidatorBuilder {
    keyrings: Vec<PathBuf>,
}

/// Struct to decode signed Debian control files.
#[derive(Clone)]
pub struct OpenPgpValidator {
    keys: HashMap<Fingerprint, Cert>,
}

/// Error conditions which may be encountered when working with OpenPGP
/// signed data within the context of the [crate::control] module.
#[derive(Debug)]
pub enum OpenPgpValidatorError {
    /// No OpenPGP keyrings were provided, so no validation is possible.
    NoConfiguredKeyring,

    /// Returned if no valid signatures were found.
    NoValidSignatures,

    /// Underlying i/o error.
    Io(std::io::Error),

    /// Underlying issue with the [sequoia_openpgp] crate.
    Sequoia(anyhow::Error),
}

impl OpenPgpValidator {
    /// Return a new [OpenPgpValidatorBuilder].
    pub fn build() -> OpenPgpValidatorBuilder {
        Default::default()
    }

    /// Take some pile of bytes, and return the valid fingerprints, as well
    /// as the signed data.
    ///
    /// This is a (very) high-level abstraction, so that we can change underlying
    /// OpenPgp implementation(s) as required down the road. We just want to
    /// give it some clearsigned text, and get back the valid signatures.
    pub fn validate(
        &self,
        message: &[u8],
    ) -> Result<(Vec<Fingerprint>, impl Read), OpenPgpValidatorError> {
        let p = &StandardPolicy::new();

        struct Helper<'a> {
            validator: &'a OpenPgpValidator,
            results: Vec<Fingerprint>,
        }

        impl VerificationHelper for &mut Helper<'_> {
            fn get_certs(&mut self, _ids: &[KeyHandle]) -> SequoiaResult<Vec<Cert>> {
                Ok(self.validator.keys.values().cloned().collect())
            }

            fn check(&mut self, structure: MessageStructure) -> SequoiaResult<()> {
                for (i, layer) in structure.into_iter().enumerate() {
                    match layer {
                        MessageLayer::Encryption { .. } if i == 0 => (),
                        MessageLayer::Compression { .. } if i == 1 => (),
                        MessageLayer::SignatureGroup { results } => {
                            for result in results {
                                let Ok(result) = result else {
                                    continue;
                                };

                                self.results
                                    .extend(result.sig.issuer_fingerprints().cloned());
                            }
                        }
                        _ => return Err(anyhow::anyhow!("Unexpected message structure")),
                    }
                }
                Ok(())
            }
        }

        let mut helper = Helper {
            validator: self,
            results: vec![],
        };

        let mut v = VerifierBuilder::from_bytes(message)
            .map_err(OpenPgpValidatorError::Sequoia)?
            .with_policy(p, None, &mut helper)
            .map_err(OpenPgpValidatorError::Sequoia)?;

        let mut content = vec![];
        v.read_to_end(&mut content)
            .map_err(OpenPgpValidatorError::Io)?;

        let Helper { results, .. } = helper;

        if results.is_empty() {
            return Err(OpenPgpValidatorError::NoValidSignatures);
        }

        Ok((results, Cursor::new(content)))
    }
}

impl OpenPgpValidatorBuilder {
    /// Use the provided keyring. This will append the provided keyring
    /// to the set of authorized keys, rather than replacing.
    pub fn with_keyring(mut self, path: &Path) -> Self {
        self.keyrings.push(path.to_owned());
        self
    }

    /// Build the provided OpenPgpValidator.
    pub fn build(self) -> Result<OpenPgpValidator, OpenPgpValidatorError> {
        let keys = {
            let mut keys = HashMap::new();
            for keyring in self.keyrings {
                for cert in
                    CertParser::from_file(keyring).map_err(OpenPgpValidatorError::Sequoia)?
                {
                    let cert = cert.map_err(OpenPgpValidatorError::Sequoia)?;
                    keys.insert(cert.fingerprint(), cert);
                }
            }
            keys
        };

        Ok(OpenPgpValidator { keys })
    }
}

// vim: foldmethod=marker
