#![no_main]

use deb::dependency::Dependency;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = data.parse::<Dependency>();
});
