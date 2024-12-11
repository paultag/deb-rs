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

//! Module to use the [serde] deserialization framework to decode data in
//! Debian's RFC2822-like format to Rust types.

use crate::control::RawParagraph;
use serde::de;
use std::{
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
};

#[cfg(feature = "sequoia")]
use crate::control::openpgp::{self, OpenPgpValidatorError};

#[cfg(feature = "sequoia")]
use sequoia_openpgp::Fingerprint;

#[cfg(feature = "sequoia")]
use std::path::Path;

mod outer;
mod paragraph;

/// Error conditions which may be encountered during Deserialization
#[derive(Debug)]
pub enum Error {
    /// Generic pass-through error from Serde. These are generally
    /// fairly unhelpful, and hard to match meaningfully on. If you get
    /// one of these, best bet is to hard bail.
    De(String),

    /// The end of the file was reached before it was expected. This tends
    /// to be returned if something was syntactically wrong.
    EndOfFile,

    /// A number in the control file could not be turned into a number
    /// from the Field's String value.
    InvalidNumber,

    /// A boolean in the control file could not be turned into a boolean
    /// from the Field's String value.
    InvalidBool,

    /// Error parsing the underlying [RawParagraph].
    ParseError(crate::control::Error),

    /// The [crate::control::de] module is very *very* basic. No effort
    /// is made to decode fancy enums or `Vec` types as of yet, so
    /// it's extremely likely that you'll see this if you're pushing
    /// the bounds of what this crate can do.
    BadType,

    /// Underlying transport issue generally caused by some i/o boundary.
    Io(std::io::Error),

    #[cfg(feature = "sequoia")]
    /// Error encountered doing GPG Validation.
    OpenPgp(OpenPgpValidatorError),

    /// Somehow, against all odds, something managed to be invalid
    /// Utf-8, and was caught astonishingly late in the process.
    InvalidText(std::str::Utf8Error),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Self::Io(ioe)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl de::Error for Error {
    fn custom<T>(custom: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::De(custom.to_string())
    }
}

/// Deserialize the provided Debian-flavored RFC2822 data into the desired
/// Rust type from a [std::io::Read].
///
/// ```no_run
/// use deb::control::{de, changes::Changes};
///
/// // Read a .changes file from stdin
///
/// let mut stdin = std::io::BufReader::new(std::io::stdin());
/// let changes: Changes = de::from_reader(&mut stdin).unwrap();
///
/// println!("{:?}", changes);
/// ```
pub fn from_reader<'a, 'de, T, ReadT>(input: &'a mut BufReader<ReadT>) -> Result<T, Error>
where
    ReadT: Read,
    T: de::Deserialize<'de>,
{
    let mut buf = String::new();

    loop {
        match input.read_line(&mut buf)? {
            0 => {
                if buf.trim().is_empty() {
                    return Err(Error::EndOfFile);
                }
                return from_str(&buf);
            }
            1 => {
                // if we pushed back a single char
                if buf.ends_with('\n') && !buf.trim().is_empty() {
                    // if we pushed a newline and we have something other than
                    // whitespace, lets go and decode. Otherwise we're still
                    // in the leadup maybe.
                    return from_str(&buf);
                }
            }
            _ => {}
        }
    }
}

struct ControlIterator<'a, 'de, T, ReadT> {
    input: &'a mut BufReader<ReadT>,
    _de: PhantomData<&'de ()>,
    _t: PhantomData<T>,
}

impl<'a, 'de, T, ReadT> Iterator for ControlIterator<'a, 'de, T, ReadT>
where
    ReadT: Read,
    T: de::Deserialize<'de>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match from_reader(self.input) {
            Err(Error::EndOfFile) => None,
            v => Some(v),
        }
    }
}

/// Return an iterator
pub fn from_reader_iter<'a, 'de, T, ReadT>(
    input: &'a mut BufReader<ReadT>,
) -> impl Iterator<Item = Result<T, Error>> + use<'a, 'de, T, ReadT>
where
    ReadT: Read,
    T: de::Deserialize<'de>,
{
    ControlIterator {
        input,
        _de: PhantomData,
        _t: PhantomData,
    }
}

#[cfg(feature = "tokio")]
mod _tokio {
    use super::*;
    use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

    /// Deserialize the provided Debian-flavored RFC2822 data into the desired
    /// Rust type from a [tokio::io::AsyncRead].
    ///
    /// ```no_run
    /// use deb::control::{de, changes::Changes};
    ///
    /// async fn read_changes() {
    ///     // input can be any AsyncRead, as long as it's wrapped in
    ///     // a tokio BufReader. Here we use a std::io::Cursor which
    ///     // has an AsyncRead implementation on it.
    ///     let mut input = tokio::io::BufReader::new(std::io::Cursor::new(""));
    ///
    ///     let changes: Changes = de::from_reader_async(&mut input).await.unwrap();
    ///     println!("{:?}", changes);
    /// }
    /// ```
    pub async fn from_reader_async<'a, 'de, T, ReadT>(
        input: &'a mut BufReader<ReadT>,
    ) -> Result<T, Error>
    where
        ReadT: AsyncRead,
        ReadT: Unpin,
        T: de::Deserialize<'de>,
    {
        let mut buf = String::new();

        loop {
            match input.read_line(&mut buf).await? {
                0 => {
                    if buf.trim().is_empty() {
                        return Err(Error::EndOfFile);
                    }
                    return from_str(&buf);
                }
                1 => {
                    // if we pushed back a single char
                    if buf.ends_with('\n') && !buf.trim().is_empty() {
                        // if we pushed a newline and we have something other than
                        // whitespace, lets go and decode. Otherwise we're still
                        // in the leadup maybe.
                        return from_str(&buf);
                    }
                }
                _ => {}
            }
        }
    }

    /// Pretend to be an [Iterator], but async.
    ///
    /// There's an unpleasantry of traits which are currently in the
    /// process of exiting the Thunderdome, but none have been declared
    /// the winner yet. None of the traits in `std` are out of nightly
    /// yet, `tokio` took a lax stance in the `tokio-streams` crate,
    /// and all of them are a mess to implement and maintain, since they
    /// implement the raw promise-like `poll` interface rather than
    /// using something like `impl Future`.
    ///
    /// As a result, I'm going to avoid trying to deal with that ecosystem
    /// as best as I can until something comes out as the winner, at which
    /// point I'll sprinkle in some implementations and turn this struct
    /// into a private type, and use an `impl Stream` return type from
    /// [from_reader_async_iter].
    ///
    /// Until then this struct will behave like you want -- generally, speaking.
    /// The downside of this decision is that things like `StreamExt` won't
    /// work on this struct as-is.
    pub struct AsyncControlIterator<'a, 'de, T, ReadT>
    where
        ReadT: AsyncRead,
        ReadT: Unpin,
        T: de::Deserialize<'de>,
    {
        input: &'a mut BufReader<ReadT>,
        _de: PhantomData<&'de ()>,
        _t: PhantomData<T>,
    }

    impl<'a, 'de, T, ReadT> AsyncControlIterator<'a, 'de, T, ReadT>
    where
        ReadT: AsyncRead,
        ReadT: Unpin,
        T: de::Deserialize<'de>,
    {
        /// Normal [Iterator]-like protocol -- return a None to indicate
        /// the end of the stream has been reached, otherwise, return
        /// a `Some` containing the same return type you'd get from
        /// [from_reader_async].
        pub async fn next(&mut self) -> Option<Result<T, Error>> {
            match from_reader_async(self.input).await {
                Err(Error::EndOfFile) => None,
                v => Some(v),
            }
        }
    }

    /// Deserialize a repeated sequence of Debian-flavored RFC2822 control
    /// paragraphs into the desired Rust types from a [tokio::io::AsyncRead].
    ///
    /// This can't be an [Iterator], since those are not async. We could
    /// target a `tokio` stream, but it's fairly low level and not fully
    /// stable yet. Same for a `std::async_iter::AsyncIterator`.
    ///
    /// All of this adds a level of congnitive overhead on using this module
    /// that's not worth it as of yet, especially since we can implement
    /// the trait in the future. For now, we're returning the
    /// [AsyncControlIterator] type, which will behave like an async
    /// iterator should.
    ///
    /// ```no_run
    /// use deb::control::{de, changes::Changes};
    ///
    /// async fn read_changes() {
    ///     // input can be any AsyncRead, as long as it's wrapped in
    ///     // a tokio BufReader. Here we use a std::io::Cursor which
    ///     // has an AsyncRead implementation on it.
    ///     let mut input = tokio::io::BufReader::new(std::io::Cursor::new(""));
    ///
    ///     let mut iter = de::from_reader_async_iter::<Changes, _>(&mut input);
    ///     while let Some(changes) = iter.next().await {
    ///         let changes = changes.unwrap();
    ///         println!("{:?}", changes);
    ///     }
    /// }
    /// ```
    pub fn from_reader_async_iter<'a, 'de, T, ReadT>(
        input: &'a mut BufReader<ReadT>,
    ) -> AsyncControlIterator<'a, 'de, T, ReadT>
    where
        ReadT: AsyncRead,
        ReadT: Unpin,
        T: de::Deserialize<'de>,
    {
        AsyncControlIterator {
            input,
            _de: PhantomData,
            _t: PhantomData,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde::Deserialize;
        use std::io::Cursor;

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct TestControl {
            #[serde(rename = "Hello")]
            hello: String,
        }

        #[tokio::test]
        async fn test_from_reader_async() {
            let mut reader = BufReader::new(Cursor::new(
                "\
Hello: World
",
            ));

            let test: TestControl = from_reader_async(&mut reader).await.unwrap();
            assert_eq!(test.hello, "World");
        }

        #[tokio::test]
        async fn test_from_reader_async_iter() {
            let mut reader = BufReader::new(Cursor::new(
                "\
Hello: World

Hello: Paul

Hello: You

Hello: Me
",
            ));

            let mut iter = from_reader_async_iter(&mut reader);

            let mut values = vec![];
            while let Some(rv) = iter.next().await {
                let rv: TestControl = rv.unwrap();
                values.push(rv.hello);
            }
            assert_eq!(vec!["World", "Paul", "You", "Me"], values);
        }
    }
}

#[cfg(feature = "tokio")]
pub use _tokio::{from_reader_async, from_reader_async_iter, AsyncControlIterator};

/// Check the signature of a clearsigned OpenPGP signature against the provided
/// keyring, and if the signature is good, parse and return the signed
/// data, along with any valid signatures.
///
/// # Note ♫
///
/// This requires the `sequoia` feature.
#[cfg(feature = "sequoia")]
pub fn from_clearsigned_str<'a, 'de, T>(
    keyring: &Path,
    input: &'a str,
) -> Result<(Vec<Fingerprint>, T), Error>
where
    T: de::Deserialize<'de>,
{
    let (fingerprints, input) = openpgp::verify(keyring, input).map_err(Error::OpenPgp)?;
    Ok((fingerprints, from_reader(&mut BufReader::new(input))?))
}

/// Return the parsed control file from the input string.
pub fn from_str<'a, 'de, T>(input: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let input = input.trim_start();
    let rp = RawParagraph::parse(input).map_err(Error::ParseError)?;
    from_raw_paragraph(&rp)
}

/// Decode from a [RawParagraph]
fn from_raw_paragraph<'a, 'de, T>(input: &'a RawParagraph) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let iter = input
        .fields
        .iter()
        .flat_map(|v| [v.key.as_str(), v.value.as_str()])
        .peekable();
    let mut deserializer = outer::Deserializer { iter };
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::Version;
    use serde::Deserialize;
    use std::io::Cursor;

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    struct TestControlFile {
        #[serde(rename = "Package")]
        package: String,

        #[serde(rename = "Foo")]
        foo: String,

        #[serde(rename = "True-False")]
        true_false: bool,

        #[serde(rename = "X-A-Number")]
        a_number: usize,

        #[serde(rename = "Ello")]
        ello: Option<String>,
    }

    macro_rules! test_de_fails_with {
        ($name:ident, $type:ty) => {
            #[test]
            fn $name() {
                let err = from_str::<$type>(
                    "\
Package: something
Foo: Bar
True-False: true
",
                )
                .err()
                .unwrap();

                assert!(
                    matches!(err, Error::BadType),
                    "expected {} got {}",
                    Error::BadType,
                    err
                );
            }
        };
    }

    test_de_fails_with!(test_to_i8, i8);
    test_de_fails_with!(test_to_i16, i16);
    test_de_fails_with!(test_to_i32, i32);
    test_de_fails_with!(test_to_i64, i64);
    test_de_fails_with!(test_to_i128, i128);

    test_de_fails_with!(test_to_u8, u8);
    test_de_fails_with!(test_to_u16, u16);
    test_de_fails_with!(test_to_u32, u32);
    test_de_fails_with!(test_to_u64, u64);
    test_de_fails_with!(test_to_u128, u128);

    test_de_fails_with!(test_to_f32, f32);
    test_de_fails_with!(test_to_f64, f64);

    test_de_fails_with!(test_to_bool, bool);
    test_de_fails_with!(test_to_vec, Vec<String>);
    test_de_fails_with!(test_to_string, String);

    #[test]
    fn test_basic_types_option_some() {
        let test: TestControlFile = from_str(
            "\
Package: something
Foo: Bar
True-False: true
X-A-Number: 10
",
        )
        .unwrap();

        assert_eq!(test.package, "something");
        assert_eq!(test.foo, "Bar");
        assert!(test.true_false);
        assert_eq!(test.a_number, 10);
        assert!(test.ello.is_none());
    }

    #[test]
    fn test_basic_type_option_none() {
        let test: TestControlFile = from_str(
            "\
Package: something
Foo: Bar
True-False: true
X-A-Number: 10
Ello: something
        ",
        )
        .unwrap();

        assert_eq!(test.package, "something");
        assert_eq!(test.foo, "Bar");
        assert!(test.true_false);
        assert_eq!(test.a_number, 10);
        assert!(test.ello.is_some());
    }

    #[test]
    fn test_reader() {
        assert!(from_reader::<TestControlFile, _>(&mut BufReader::new(Cursor::new(""))).is_err());
        assert!(
            from_reader::<TestControlFile, _>(&mut BufReader::new(Cursor::new(
                "



"
            )))
            .is_err()
        );
    }

    #[test]
    fn test_reader_multi() {
        let mut reader = BufReader::new(Cursor::new(
            "\
Package: somethingelse
Foo: Foo
True-False: false
X-A-Number: 100000

Package: sth
Foo: Foo1
True-False: true
X-A-Number: 10000


Package: else
Foo: Foo2
True-False: false
X-A-Number: 1000
Ello: Govnr
",
        ));

        let test: TestControlFile = from_reader(&mut reader).unwrap();
        assert_eq!(test.package, "somethingelse");
        assert_eq!(test.foo, "Foo");
        assert!(!test.true_false);
        assert_eq!(test.a_number, 100000);
        assert!(test.ello.is_none());

        let test: TestControlFile = from_reader(&mut reader).unwrap();
        assert_eq!(test.package, "sth");
        assert_eq!(test.foo, "Foo1");
        assert!(test.true_false);
        assert_eq!(test.a_number, 10000);
        assert!(test.ello.is_none());

        let test: TestControlFile = from_reader(&mut reader).unwrap();
        assert_eq!(test.package, "else");
        assert_eq!(test.foo, "Foo2");
        assert!(!test.true_false);
        assert_eq!(test.a_number, 1000);
        assert!(test.ello.is_some());
    }

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    struct TestControlSecond {
        #[serde(rename = "Neato")]
        neato: String,
    }

    #[test]
    fn test_reader_mixed() {
        let mut reader = BufReader::new(Cursor::new(
            "\
Package: somethingelse
Foo: Foo
True-False: false
X-A-Number: 100000

Neato: Here
",
        ));

        let test: TestControlFile = from_reader(&mut reader).unwrap();
        assert_eq!(test.package, "somethingelse");
        assert_eq!(test.foo, "Foo");
        assert!(!test.true_false);
        assert_eq!(test.a_number, 100000);
        assert!(test.ello.is_none());

        let test: TestControlSecond = from_reader(&mut reader).unwrap();
        assert_eq!(test.neato, "Here");
    }

    #[test]
    fn test_nested_flatten() {
        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Inner {
            #[serde(rename = "Testing")]
            testing: String,
        }

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Outer {
            #[serde(flatten)]
            inner: Inner,
        }

        let test: Outer = from_str(
            "\
Testing: One
",
        )
        .unwrap();

        assert_eq!("One", test.inner.testing);
    }

    #[test]
    fn test_nested_breaks() {
        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Inner {
            #[serde(rename = "Deep")]
            deep: String,
        }

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Outer {
            #[serde(rename = "Inner")]
            inner: Inner,
        }

        assert!(matches!(
            from_str::<Outer>(
                "\
Inner: One
Deep: 2
Testing2: 1
",
            )
            .err()
            .unwrap(),
            Error::BadType
        ))
    }

    #[test]
    fn test_multiline() {
        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Multiline {
            #[serde(rename = "Multiline")]
            multiline: Vec<String>,
        }

        let ml: Multiline = from_str(
            "\
Multiline:
 Something
 Here
 And
 Here
",
        )
        .unwrap();

        assert_eq!(4, ml.multiline.len());
    }

    #[test]
    fn test_multiline_custom() {
        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct Multiline {
            #[serde(rename = "Multiline")]
            multiline: Vec<Version>,
        }

        let ml: Multiline = from_str(
            "\
Multiline:
 1.0
 1.1
 1.2
 1.3
 1.4
",
        )
        .unwrap();

        assert_eq!(
            vec!["1.0", "1.1", "1.2", "1.3", "1.4",]
                .into_iter()
                .map(|v| v.parse::<Version>().unwrap())
                .collect::<Vec<_>>(),
            ml.multiline,
        )
    }

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    struct TestControl {
        #[serde(rename = "Hello")]
        hello: String,
    }

    #[test]
    fn test_from_reader() {
        let mut reader = BufReader::new(Cursor::new(
            "\
Hello: World
",
        ));

        let test: TestControl = from_reader(&mut reader).unwrap();
        assert_eq!(test.hello, "World");
    }

    #[test]
    fn test_from_reader_iter() {
        let mut reader = BufReader::new(Cursor::new(
            "\
Hello: World

Hello: Paul

Hello: You

Hello: Me
",
        ));

        let values = from_reader_iter::<TestControl, _>(&mut reader)
            .map(|v| v.unwrap().hello)
            .collect::<Vec<_>>();
        assert_eq!(vec!["World", "Paul", "You", "Me"], values);
    }
}

// vim: foldmethod=marker
