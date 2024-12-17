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

use crate::control::{package, DigestMd5, DigestSha256};

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// Debian archive Package index file, as seen in
/// `dists/unstable/main/binary-amd64/Packages.xz` and friends.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct Package {
    /// Binary control entry from the `DEBIAN/control` file.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub control: package::BinaryControl,

    /// MD5 hash of the `.deb` file.
    ///
    /// Note: The MD5 checksum is considered weak, and should never be assumed
    /// to be sufficient for secure verification.
    #[cfg_attr(feature = "serde", serde(rename = "MD5sum"))]
    pub md5sum: DigestMd5,

    /// SHA256 hash of the `.deb` file.
    #[cfg_attr(feature = "serde", serde(rename = "SHA256"))]
    pub sha256: DigestSha256,

    /// Path within the Debian archive to the specific `.deb` file.
    pub filename: String,

    /// Size of the binary `.deb` file.
    pub size: usize,

    /// MD5 hash of the package's full Description. The `description`
    /// field only contains the short description.
    #[cfg_attr(feature = "serde", serde(rename = "Description-md5"))]
    pub description_md5: DigestMd5,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use super::*;

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;
        use crate::{architecture, control::de};

        macro_rules! test_package {
            ($name:ident, $data:expr, |$parsed:ident| $block:tt) => {
                #[test]
                fn $name() {
                    let $parsed = de::from_str::<Package>($data).unwrap();
                    $block
                }
            };
        }

        test_package!(parse_simple, "\
Package: fluxbox
Source: fluxbox (1.3.7-1)
Version: 1.3.7-1+b1
Installed-Size: 4128
Maintainer: Dmitry E. Oboukhov <unera@debian.org>
Architecture: amd64
Provides: x-window-manager
Depends: menu (>= 2.1.19), libc6 (>= 2.34), libfontconfig1 (>= 2.12.6), libfribidi0 (>= 0.19.2), libgcc-s1 (>= 3.0), libimlib2t64 (>= 1.4.5), libstdc++6 (>= 13.1), libx11-6, libxext6, libxft2 (>> 2.1.1), libxinerama1 (>= 2:1.1.4), libxpm4, libxrandr2, libxrender1
Recommends: xfonts-terminus, feh | eterm | hsetroot | xloadimage
Suggests: fbpager, fbdesk, fbautostart
Description: Highly configurable and low resource X11 Window manager
Homepage: https://fluxbox.org
Description-md5: 13990cdf4dc1b2dc117250b7023f2e58
Tag: implemented-in::c, interface::graphical, interface::x11, role::program,
 scope::application, uitoolkit::gtk, x11::window-manager
Section: x11
Priority: optional
Filename: pool/main/f/fluxbox/fluxbox_1.3.7-1+b1_amd64.deb
Size: 1226140
MD5sum: e9ae48ab62d609faaafdd034353a28d7
SHA256: 7eaf5da83ab47fce0937b348640aec52c96ae5193b809d01168c5c81bd7f4645
", |package| {
            assert_eq!("fluxbox", package.control.package);
            assert_eq!(architecture::AMD64, package.control.architecture.unwrap());
            // assert_eq!(4128, package.control.installed_size.unwrap());
            assert!(package.control.depends.is_some());
            assert!(package.control.recommends.is_some());
            assert!(package.control.suggests.is_some());
            assert!(package.control.conflicts.is_none());
            assert_eq!(4128, *package.control.installed_size.unwrap());
            assert_eq!("pool/main/f/fluxbox/fluxbox_1.3.7-1+b1_amd64.deb", package.filename);
            assert_eq!(1226140, package.size);
        });
    }
}

// vim: foldmethod=marker
