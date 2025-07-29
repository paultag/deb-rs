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

//! Rust types to handle Deserialization of a Debian archive files.

mod binary_control;
mod buildinfo;
mod changes;
mod common_source_control;
mod dsc;
mod file;
mod package_list;
mod source_control;
mod source_name;

pub use binary_control::BinaryControl;
pub use buildinfo::Buildinfo;
pub use changes::{Changes, ChangesParseError};
pub use common_source_control::CommonSourceControl;
pub use dsc::{Dsc, DscParseError};
pub use file::File;
pub use package_list::PackageList;
pub use source_control::SourceControl;
pub use source_name::{SourceName, SourceNameError};

// vim: foldmethod=marker
