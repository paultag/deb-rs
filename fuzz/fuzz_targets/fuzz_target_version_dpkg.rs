#![no_main]

use deb::version::Version;
use libfuzzer_sys::fuzz_target;
use std::{cmp::Ordering, process::Command};

fn cmp2cli(o: Ordering) -> &'static str {
    match o {
        Ordering::Equal => "eq",
        Ordering::Less => "lt",
        Ordering::Greater => "gt",
    }
}

fn check_cmp_dpkg(v1: &Version, ord: Ordering, v2: &Version) -> bool {
    Command::new("dpkg")
        .arg("--compare-versions")
        .arg(v1.to_string())
        .arg(cmp2cli(ord))
        .arg(v2.to_string())
        .output()
        .unwrap()
        .status
        .success()
}

fuzz_target!(|data: &str| {
    let Ok(v1) = data.parse::<Version>() else {
        return;
    };
    let v2: Version = "100:100.100+100-100onehundred100~100".parse().unwrap();

    let _ = v1.cmp(&v2);
    let _ = v2.cmp(&v1);

    assert!(check_cmp_dpkg(&v1, v1.cmp(&v2), &v2));
    assert!(check_cmp_dpkg(&v2, v2.cmp(&v1), &v1));
});
