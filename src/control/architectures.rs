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

use crate::{architecture::Architecture, control::Delimited};

/// List of [Architecture] values, seperated with a space.
pub type Architectures = Delimited<' ', Architecture>;

#[cfg(test)]
mod tests {
    use super::Architectures;
    use crate::{
        architecture::{self, Architecture},
        control::{def_failing_parse_test, def_parse_test, Delimited},
    };

    def_parse_test!(
        parse_empty,
        Architectures,
        "",
        Delimited::<' ', Architecture>(vec![])
    );
    def_parse_test!(
        parse_amd64,
        Architectures,
        "amd64",
        Delimited::<' ', Architecture>(vec![architecture::AMD64])
    );
    def_parse_test!(
        parse_amd64_arm64,
        Architectures,
        "amd64 arm64",
        Delimited::<' ', Architecture>(vec![architecture::AMD64, architecture::ARM64])
    );
    def_failing_parse_test!(
        parse_amd64_arm64_with_spaces,
        Architectures,
        "amd64    arm64"
    );

    def_failing_parse_test!(fail_bad_arch, Architectures, "foo-bar-baz-bar-foo");
}

// vim: foldmethod=marker
