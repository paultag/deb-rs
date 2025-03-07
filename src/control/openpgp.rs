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

#![cfg_attr(docsrs, doc(cfg(feature = "sequoia")))]

use sequoia_openpgp::{
    Cert, Fingerprint, KeyHandle, Result as SequoiaResult,
    cert::CertParser,
    packet::Signature,
    parse::{
        Parse,
        stream::{MessageLayer, MessageStructure, VerificationHelper, VerifierBuilder},
    },
    policy::StandardPolicy,
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
    insecure_skip_verify: bool,
}

/// Struct to decode signed Debian control files.
#[derive(Clone)]
pub struct OpenPgpValidator {
    keys: HashMap<Fingerprint, Cert>,
    insecure_skip_verify: bool,
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
crate::errors::error_enum!(OpenPgpValidatorError);

/// Wrapper type for a `Vec` of [Cert] and [Signature].
pub type Signatures = Vec<(Cert, Signature)>;

impl OpenPgpValidator {
    /// Return a new [OpenPgpValidatorBuilder].
    pub fn build() -> OpenPgpValidatorBuilder {
        Default::default()
    }

    /// Load the contents of the provided Read traited object into memory,
    /// and run it through [Self::validate].
    pub fn validate_reader<ReadT>(
        &self,
        mut message: ReadT,
    ) -> Result<(Signatures, Cursor<Vec<u8>>), OpenPgpValidatorError>
    where
        ReadT: Read,
    {
        let mut bytes = vec![];
        message
            .read_to_end(&mut bytes)
            .map_err(OpenPgpValidatorError::Io)?;
        self.validate(&bytes)
    }

    /// Take some pile of bytes, and return the valid fingerprints, as well
    /// as the signed data.
    ///
    /// This is a (very) high-level abstraction, so that we can change underlying
    /// OpenPgp implementation(s) as required down the road. We just want to
    /// give it some clearsigned text, and get back the valid signatures.
    ///
    /// Currently this has to load the whole file into memory. This sucks, but
    /// it's an unfortunate reality; especially since we need to read the
    /// whole file to validate (hash) it, this may be something we just need
    /// to eat. This means we may want to do some sort of bounds checking before
    /// we get here.
    pub fn validate(
        &self,
        message: &[u8],
    ) -> Result<(Signatures, Cursor<Vec<u8>>), OpenPgpValidatorError> {
        let p = &StandardPolicy::new();

        struct Helper<'a> {
            validator: &'a OpenPgpValidator,
            results: Signatures,
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

                                let signature = result.sig.clone();
                                let fingerprints = signature.issuer_fingerprints();

                                for fingerprint in fingerprints {
                                    let Some(signer) = self.validator.keys.get(fingerprint) else {
                                        continue;
                                    };
                                    self.results.push((signer.clone(), signature.clone()));
                                }
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

        if results.is_empty() && !self.insecure_skip_verify {
            return Err(OpenPgpValidatorError::NoValidSignatures);
        }

        Ok((results, Cursor::new(content)))
    }
}

#[cfg(feature = "tokio")]
mod _tokio {
    #![cfg_attr(docsrs, doc(cfg(feature = "tokio")))]

    use super::*;
    use tokio::io::{AsyncRead, AsyncReadExt};

    impl OpenPgpValidator {
        /// Load the contents of the provided AsyncRead traited object into memory,
        /// and run it through [Self::validate].
        pub async fn validate_reader_async<ReadT>(
            &self,
            mut message: ReadT,
        ) -> Result<(Vec<(Cert, Signature)>, Cursor<Vec<u8>>), OpenPgpValidatorError>
        where
            ReadT: AsyncRead,
            ReadT: Unpin,
        {
            let mut bytes = vec![];
            message
                .read_to_end(&mut bytes)
                .await
                .map_err(OpenPgpValidatorError::Io)?;
            self.validate(&bytes)
        }
    }
}

impl OpenPgpValidatorBuilder {
    /// Use the provided keyring. This will append the provided keyring
    /// to the set of authorized keys, rather than replacing.
    pub fn with_keyring(mut self, path: &Path) -> Self {
        self.keyrings.push(path.to_owned());
        self
    }

    /// DO NOT USE THIS OUTSIDE TESTING
    pub fn with_insecure_skip_verify_this_is_a_bad_idea(mut self) -> Self {
        self.insecure_skip_verify = true;
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
                    keys.insert(cert.fingerprint(), cert.clone());
                    for key in cert.keys() {
                        keys.insert(key.key().fingerprint(), cert.clone());
                    }
                }
            }
            keys
        };

        Ok(OpenPgpValidator {
            keys,
            insecure_skip_verify: self.insecure_skip_verify,
        })
    }
}

/// This is a thin wrapper around the [OpenPgpValidator], but with all of the
/// moving parts possible removed. This is going to be an interface that we
/// can make a bit more generic than the concrete [OpenPgpValidator] interface,
/// so this should be used when possible.
#[cfg_attr(not(feature = "serde"), allow(dead_code))]
pub(crate) fn verify(
    keyring: &Path,
    input: &str,
) -> Result<(Vec<(Cert, Signature)>, impl Read), OpenPgpValidatorError> {
    let verifier = OpenPgpValidator::build().with_keyring(keyring).build()?;
    verifier.validate(input.as_bytes())
}

// vim: foldmethod=marker
