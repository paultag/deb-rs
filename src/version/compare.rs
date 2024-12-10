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

//! The `version` module implements support for comparing Versions.

// This is a from-docs reimplementation of the algorithm described in
// `deb-version(5)`.

use super::{Error, Version};
use std::{cmp::Ordering, str::Chars};

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.epoch().unwrap_or(0).cmp(&other.epoch().unwrap_or(0)) {
            Ordering::Equal => {}
            v => return v,
        };

        match compare_version_str(self.upstream_version(), other.upstream_version()) {
            Ordering::Equal => {}
            v => return v,
        }

        compare_version_str(
            self.debian_revision().unwrap_or("0"),
            other.debian_revision().unwrap_or("0"),
        )
    }
}

/// Used internally to parse dpkg versions
struct VersionCompareIterator<'version> {
    _version: &'version str,
    it: Chars<'version>,
    ch: Option<char>,

    // true if we're extracting a string, false if we're extracting
    // a digit.
    in_string: bool,
}

impl<'version> VersionCompareIterator<'version> {
    fn new(version: &'version str) -> Self {
        let mut it = version.chars();
        Self {
            _version: version,
            ch: it.next(),
            it,

            // according to the dpkg spec, we start off in string mode.
            in_string: true,
        }
    }
}

/// VersionComponent
#[derive(Clone, Debug, PartialEq)]
enum VersionComponent {
    String(String),
    Number(String),
}

fn is_version_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_version_string(ch: char) -> bool {
    ch.is_ascii_lowercase()
        || ch.is_ascii_uppercase()
        || ch == '.'
        || ch == '-'
        || ch == '+'
        || ch == ':'
        || ch == '~'
}

fn version_char_to_num(ch: char) -> usize {
    if !is_version_string(ch) {
        panic!("malformed char");
    }

    if ch == '~' {
        // ~ is always the smallest.
        return 0;
    }

    // default to the ascii range for normal numbers
    if ch.is_ascii_lowercase() || ch.is_ascii_uppercase() {
        // if we're a-z, A-Z, return them as-is.
        return ch as usize;
    }

    // here we know we have one of .-+:~, which we need to map from
    // 46, 45, 43, 58, 126 to values larger than Z, but in the same
    // order.

    let rv = ch as usize;

    // 90 is "Z".
    rv + 90
}

fn compare_version_char(left: char, right: char) -> Ordering {
    version_char_to_num(left).cmp(&version_char_to_num(right))
}

fn compare_version_str(left: &str, right: &str) -> Ordering {
    let left_it = VersionCompareIterator::new(left)
        .flatten()
        .collect::<Vec<_>>()
        .into_iter();

    let right_it = VersionCompareIterator::new(right)
        .flatten()
        .collect::<Vec<_>>()
        .into_iter();

    for (left_component, right_component) in left_it.zip(right_it) {
        let cmp = match (left_component, right_component) {
            (VersionComponent::String(left_str), VersionComponent::String(right_str)) => {
                compare_version_str_component(&left_str, &right_str)
            }
            (VersionComponent::Number(left_dig), VersionComponent::Number(right_dig)) => {
                compare_version_number_component(&left_dig, &right_dig)
            }
            _ => {
                panic!("the author of this crate done fucked up");
            }
        };

        match cmp {
            Ordering::Equal => {}
            v => return v,
        }
    }

    Ordering::Equal
}

fn compare_version_number_component(left: &str, right: &str) -> Ordering {
    let left = left.trim_start_matches('0');
    let right = right.trim_start_matches('0');

    match left.len().cmp(&right.len()) {
        Ordering::Equal => {}
        v => return v,
    }

    for (left_ch, right_ch) in left.chars().zip(right.chars()) {
        match left_ch.cmp(&right_ch) {
            Ordering::Equal => {}
            v => return v,
        }
    }

    Ordering::Equal
}

fn compare_version_str_component(left: &str, right: &str) -> Ordering {
    let mut last_idx = 0;
    for (idx, (left_ch, right_ch)) in left.chars().zip(right.chars()).enumerate() {
        last_idx = idx;
        match compare_version_char(left_ch, right_ch) {
            Ordering::Equal => {}
            v => return v,
        }
    }

    let mut last_ch = last_idx + 1;
    if left.is_empty() || right.is_empty() {
        last_ch = 0;
    }

    match left.len().cmp(&right.len()) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Less => {
            if right.chars().nth(last_ch).unwrap() == '~' {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        Ordering::Greater => {
            if left.chars().nth(last_ch).unwrap() == '~' {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl Iterator for VersionCompareIterator<'_> {
    type Item = Result<VersionComponent, Error>;

    fn next(&mut self) -> Option<Result<VersionComponent, Error>> {
        let mut lch: Option<char> = self.ch;

        // we're going to glob on up the next string we're matching here. We
        // need to start with the leftover from the last chunk before we move
        // on to new chars.
        if lch.is_none() {
            // if we're EOF but we ended on a number, we're going to add
            // a trailing string, so that we can properly handle something
            // like 1.0~ cmp 1.0 without tons of duplicated logic, and it won't
            // harm any cases like 1.0 cmp 1.0
            //
            // the logic here is a bit double-negative-y, so double check.
            if self.in_string {
                self.in_string = false;
                return Some(Ok(VersionComponent::String("".to_owned())));
            }
            return None;
        };

        // if we're in a string we want to get the next string component
        // or an empty string, otherwise we want to return the next number
        // component.
        let in_string = self.in_string;

        let mut matched = String::new();
        loop {
            let Some(ch) = lch else {
                break;
            };

            let is_digit = is_version_digit(ch);
            let is_string = is_version_string(ch);

            if !is_digit && !is_string {
                return Some(Err(Error::Malformed));
            }

            if (in_string && is_digit) || (!in_string && is_string) {
                break;
            }

            matched.push(ch);
            lch = self.it.next();
        }

        self.ch = lch;
        self.in_string = !self.in_string;

        Some(Ok(if in_string {
            VersionComponent::String(matched)
        } else {
            VersionComponent::Number(matched)
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_cmp {
        ($name:ident, $version1:expr, $version2:expr, $check:expr) => {
            #[test]
            fn $name() {
                let v1: Version = $version1.parse().unwrap();
                let v2: Version = $version2.parse().unwrap();
                let cmp = v1.cmp(&v2);
                assert_eq!(
                    $check, cmp,
                    "{} should be {:?} than {} but is reported as {:?}",
                    v1, $check, v2, cmp
                );
            }
        };
    }

    #[test]
    fn check_collect() {
        let v: Version = "1.2.3foo+bar~1~".parse().unwrap();

        let version_string = v.to_string();
        let v_it = VersionCompareIterator::new(&version_string);

        assert_eq!(
            vec![
                VersionComponent::String("".to_owned()),
                VersionComponent::Number("1".to_owned()),
                VersionComponent::String(".".to_owned()),
                VersionComponent::Number("2".to_owned()),
                VersionComponent::String(".".to_owned()),
                VersionComponent::Number("3".to_owned()),
                VersionComponent::String("foo+bar~".to_owned()),
                VersionComponent::Number("1".to_owned()),
                VersionComponent::String("~".to_owned()),
            ],
            v_it.flatten().collect::<Vec<_>>()
        );
    }

    check_cmp!(cmp_simple_eq, "1.0", "1.0", Ordering::Equal);
    check_cmp!(cmp_simple_l, "1.0", "1.2", Ordering::Less);
    check_cmp!(cmp_simple_g, "1.2", "1.0", Ordering::Greater);

    check_cmp!(cmp_simple_alpha, "1a2b", "1a2b", Ordering::Equal);
    check_cmp!(cmp_simple_alpha_l, "1a2a", "1a2b", Ordering::Less);
    check_cmp!(cmp_simple_alpha_g, "1a2c", "1a2b", Ordering::Greater);

    check_cmp!(cmp_tilde, "0~~a", "0~a", Ordering::Less);
    check_cmp!(cmp_tilde_eof, "0~~", "0~", Ordering::Less);
    check_cmp!(cmp_tilde_l1, "0.1a", "0.1aa", Ordering::Less);
    check_cmp!(cmp_tilde_l2, "0.1a~", "0.1aa", Ordering::Less);
    check_cmp!(cmp_tilde_g1, "0.1aa", "0.1a", Ordering::Greater);
    check_cmp!(cmp_tilde_g2, "0.1aa", "0.1a~", Ordering::Greater);

    check_cmp!(cmp_deb_basic, "1.0-1", "1.0-1", Ordering::Equal);
    check_cmp!(cmp_deb_basic_l, "1.0-1", "1.0-2", Ordering::Less);
    check_cmp!(cmp_deb_basic_g, "1.0-2", "1.0-1", Ordering::Greater);
}

// vim: foldmethod=marker
