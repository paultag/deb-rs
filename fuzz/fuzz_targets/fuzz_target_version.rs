#![no_main]

use deb::version::Version;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let Ok(v) = data.parse::<Version>() else {
        return;
    };
    let _ = v.to_string();

    let v2: Version = "100:100.100+100-100onehundred100~100".parse().unwrap();
    let _ = v.cmp(&v2);
    let _ = v2.cmp(&v);
});
