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

/// Each package must have a priority value, which is set in the metadata for
/// the Debian archive and is also included in the package’s control files
/// (see Priority). This information is used to control which packages are
/// included in standard or minimal Debian installations.
///
/// Most Debian packages will have a priority of optional. Priority levels
/// other than optional are only used for packages that should be included by
/// default in a standard installation of Debian.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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

// vim: foldmethod=marker
