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

use std::path::PathBuf;

/// The [FileEntry] trait is implemented by structs that contain information
/// regarding a single specific file, optionally, along with a file size
/// and/or a digest of the file contents.
pub trait FileEntry {
    /// Error type returned by this trait.
    type Error;

    /// Path to the file. This is usually in relation to whatever this
    /// [FileEntry] belongs to.
    fn path(&self) -> Result<PathBuf, Self::Error>;

    /// File size, in bytes, of the underlying file referenced by this entry.
    fn size(&self) -> Option<usize>;

    /// Some digest of the File referenced by this [FileEntry], as ascii
    /// encoded hex.
    fn ascii_digest(&self) -> Option<String>;
}

// vim: foldmethod=marker
