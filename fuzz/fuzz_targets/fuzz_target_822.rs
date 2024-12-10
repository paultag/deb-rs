#![no_main]

use deb::control::RawParagraph;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = RawParagraph::parse(data);
});
