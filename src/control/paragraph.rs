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

use super::pest::{Deb822Parser, Rule};
use pest::{error::Error as PestError, iterators::Pair, Parser};

/// [RawParagraph] contains all the raw, unprocessed and fully stringified
/// [RawField] values from the underlying document. It is possible to construct
/// an invalid RawParagraph, which may create decoding or encoding issues later
/// on.
///
/// In general, using this directly is a bad idea. I've left it exported
/// to help get consumers out of a bind, but that doesn't mean I won't
/// regret that decision.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawParagraph {
    /// Raw series of fields, in the order they were seen in the Paragraph
    /// block.
    pub fields: Vec<RawField>,
}

/// Minimally processed Key-Value pair from the underlying Debian RFC2822-like
/// file.
///
/// In general, using this directly is a bad idea. I've left it exported
/// to help get consumers out of a bind, but that doesn't mean I won't
/// regret that decision.
#[derive(Clone, Debug, PartialEq)]
pub struct RawField {
    /// Key name for the Field
    pub key: String,

    /// Value of the field
    pub value: String,
}

/// Error conditions which may be encountered when working with a
/// [RawParagraph] file.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Error parsing the formatted paragraph. This is currently an exported
    /// Pest error, but that _will_ change in the future. This is only
    /// like this for testing, and will be removed.
    ///
    /// TODO: remove this
    Parse((String, pest::error::InputLocation)),

    /// Something wasn't properly encoded within the Paragraph.
    Malformed,
}
crate::errors::error_enum!(Error);

impl From<PestError<Rule>> for Error {
    fn from(err: PestError<Rule>) -> Self {
        Error::Parse((err.variant.message().into(), err.location))
    }
}

impl TryFrom<Pair<'_, Rule>> for RawField {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut key: Option<String> = None;
        let mut value = String::new();

        for part in token.into_inner() {
            match part.as_rule() {
                Rule::field_name => {
                    key = Some(part.as_str().to_owned());
                }
                Rule::field_value => {
                    value.push_str(&format!("{}\n", part.as_str()));
                }
                _ => continue,
            };
        }

        let Some(key) = key else {
            return Err(Error::Malformed);
        };

        Ok(RawField {
            key,
            value: value.trim_start_matches(' ').trim_end().to_owned(),
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for RawParagraph {
    type Error = Error;

    fn try_from(token: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut ret = Self { fields: vec![] };
        for token in token.into_inner() {
            match token.as_rule() {
                Rule::comment => {}
                Rule::field => {
                    ret.fields.push(token.try_into()?);
                }
                // TODO: validate here better
                _ => {}
            }
        }
        Ok(ret)
    }
}

impl RawParagraph {
    /// Parse a specifically formatted block of Debian flavored RFC2822 style
    /// key/value pairs, and decode it into a Paragraph. There must be not
    /// leading or trailing spaces, nor may this span multiple paragraphs.
    pub fn parse(paragraph: &str) -> Result<Self, Error> {
        let tokens = Deb822Parser::parse(Rule::single_paragraph, paragraph)?;
        let Some(token) = tokens.into_iter().next() else {
            unreachable!();
        };
        for token in token.into_inner() {
            // going to keep this since i want validation
            #[allow(clippy::single_match)]
            match token.as_rule() {
                Rule::paragraph => return token.try_into(),
                // TODO: validate here better
                _ => {}
            }
        }
        Ok(Default::default())
    }

    /// Iterate over all Key/Value pairs (as a [RawField]) in the [RawParagraph].
    pub fn iter(&self) -> impl Iterator<Item = &RawField> {
        self.fields.iter()
    }

    /// Return all matching [RawField] by the field's key.
    pub fn field<'field>(
        &'field self,
        field_name: &'field str,
    ) -> impl Iterator<Item = &'field RawField> {
        self.fields.iter().filter(move |f| f.key == field_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::control::RawParagraph;

    macro_rules! check_paragraph_parse {
        ($name:ident, $paragraph:expr, |$para:ident| $block:tt ) => {
            #[test]
            fn $name() {
                let $para = RawParagraph::parse($paragraph).unwrap();
                $block;
            }
        };
    }

    macro_rules! check_paragraph_parse_fails {
        ($name:ident, $paragraph:expr) => {
            #[test]
            fn $name() {
                assert!(RawParagraph::parse($paragraph).is_err());
            }
        };
    }

    check_paragraph_parse!(
        check_parse_comment,
        "\
Key: Value
# Comment
Key1: Value1
Key2: Value2

",
        |p| {
            assert_eq!("Value", p.field("Key").next().unwrap().value);
            assert_eq!("Value1", p.field("Key1").next().unwrap().value);
            assert_eq!("Value2", p.field("Key2").next().unwrap().value);
        }
    );

    check_paragraph_parse!(
        check_parse_comment_end,
        "\
Key: Value
Key1: Value1
Key2: Value2
# Comment
",
        |p| {
            assert_eq!("Value", p.field("Key").next().unwrap().value);
            assert_eq!("Value1", p.field("Key1").next().unwrap().value);
            assert_eq!("Value2", p.field("Key2").next().unwrap().value);
        }
    );

    check_paragraph_parse!(
        check_parse_no_nl,
        "\
Key: Value
Key1: Value1
Key2: Value2
",
        |p| {
            assert_eq!("Value", p.field("Key").next().unwrap().value);
            assert_eq!("Value1", p.field("Key1").next().unwrap().value);
            assert_eq!("Value2", p.field("Key2").next().unwrap().value);
        }
    );

    check_paragraph_parse!(
        check_parse_eof,
        "\
Key: Value
Key1: Value1
Key2: Value2",
        |p| {
            assert_eq!("Value", p.field("Key").next().unwrap().value);
            assert_eq!("Value1", p.field("Key1").next().unwrap().value);
            assert_eq!("Value2", p.field("Key2").next().unwrap().value);
        }
    );

    check_paragraph_parse!(
        check_parse_nlnlnlnl,
        "\
Key: Value
Key1: Value1
Key2: Value2



",
        |p| {
            assert_eq!("Value", p.field("Key").next().unwrap().value);
            assert_eq!("Value1", p.field("Key1").next().unwrap().value);
            assert_eq!("Value2", p.field("Key2").next().unwrap().value);
        }
    );

    check_paragraph_parse_fails!(
        check_fails_invalid_key,
        "\
Foo bar: 1
"
    );

    check_paragraph_parse_fails!(
        check_fails_two_paragraphs,
        "\
Foo: Bar

Bar: Baz
"
    );

    check_paragraph_parse!(
        check_parse_confusing_sep,
        "\
Key:Name: Value?
",
        |p| {
            assert_eq!("Name: Value?", p.field("Key").next().unwrap().value);
        }
    );
}

// vim: foldmethod=marker
