//! Test vectors from the dpkg version comparison package.
//!
//! These tests are taken from
//! [Dpkg_Version.t](Dpkg-1.20.9/source/t/Dpkg_Version.t)'s data table. I do
//! not believe the version table at the bottom is copyrightable, but in
//! the spirit of ensuring we have a clear division in case there's any licensing
//! concerns -- this is in a separate file, and not shipped in builds using
//! this libray.

#[cfg(test)]
mod test {
    use crate::version::Version;
    use std::cmp::Ordering;

    macro_rules! dpkg_tests {
        ( $( ( $name:ident, $version1:expr, $version2:expr, $check:expr ) ),* ) => {
            $(
            #[test]
            fn $name() {
                let v1: Version = $version1.parse().unwrap();
                let v2: Version = $version2.parse().unwrap();

                let check = match $check {
                    -1 => Ordering::Less,
                    1 => Ordering::Greater,
                    0 => Ordering::Equal,
                    _ => unreachable!(),
                };
                let cmp = v1.cmp(&v2);

                assert_eq!(
                    check,
                    cmp,
                    "{} should be {:?} then {}, but is reported as {:?}",
                    v1, check, v2, cmp,
                );
            }
            )*
        };
    }

    dpkg_tests![
        (test_vector_1, "1.0-1", "2.0-2", -1),
        (test_vector_2, "2.2~rc-4", "2.2-1", -1),
        (test_vector_3, "2.2-1", "2.2~rc-4", 1),
        (test_vector_4, "1.0000-1", "1.0-1", 0),
        (test_vector_5, "1", "0:1", 0),
        (test_vector_6, "0", "0:0-0", 0),
        (test_vector_7, "2:2.5", "1:7.5", 1),
        (test_vector_8, "1:0foo", "0foo", 1),
        (test_vector_9, "0:0foo", "0foo", 0),
        (test_vector_10, "0foo", "0foo", 0),
        (test_vector_11, "0foo-0", "0foo", 0),
        (test_vector_12, "0foo", "0foo-0", 0),
        (test_vector_13, "0foo", "0fo", 1),
        (test_vector_14, "0foo-0", "0foo+", -1),
        (test_vector_15, "0foo~1", "0foo", -1),
        (test_vector_16, "0foo~foo+Bar", "0foo~foo+bar", -1),
        (test_vector_17, "0foo~~", "0foo~", -1),
        (test_vector_18, "1~", "1", -1),
        (
            test_vector19,
            "12345+that-really-is-some-ver-0",
            "12345+that-really-is-some-ver-10",
            -1
        ),
        (test_vector_20, "0foo-0", "0foo-01", -1),
        (test_vector_21, "0foo.bar", "0foobar", 1),
        (test_vector_22, "0foo.bar", "0foo1bar", 1),
        (test_vector_23, "0foo.bar", "0foo0bar", 1),
        (test_vector_24, "0foo1bar-1", "0foobar-1", -1),
        (test_vector_25, "0foo2.0", "0foo2", 1),
        (test_vector_26, "0foo2.0.0", "0foo2.10.0", -1),
        (test_vector_27, "0foo2.0", "0foo2.0.0", -1),
        (test_vector_28, "0foo2.0", "0foo2.10", -1),
        (test_vector_29, "0foo2.1", "0foo2.10", -1),
        (test_vector_30, "1.09", "1.9", 0),
        (test_vector_31, "1.0.8+nmu1", "1.0.8", 1),
        (test_vector_32, "3.11", "3.10+nmu1", 1),
        (test_vector_33, "0.9j-20080306-4", "0.9i-20070324-2", 1),
        (test_vector_34, "1.2.0~b7-1", "1.2.0~b6-1", 1),
        (test_vector_35, "1.011-1", "1.06-2", 1),
        (test_vector_36, "0.0.9+dfsg1-1", "0.0.8+dfsg1-3", 1),
        (test_vector_37, "4.6.99+svn6582-1", "4.6.99+svn6496-1", 1),
        (test_vector_38, "53", "52", 1),
        (test_vector_39, "0.9.9~pre122-1", "0.9.9~pre111-1", 1),
        (test_vector_40, "2:2.3.2-2+lenny2", "2:2.3.2-2", 1),
        (test_vector_41, "1:3.8.1-1", "3.8.GA-1", 1),
        (test_vector_42, "1.0.1+gpl-1", "1.0.1-2", 1),
        (test_vector_43, "1a", "1000a", -1)
    ];
}
