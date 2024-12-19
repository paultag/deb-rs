// .dak-commands

macro_rules! test_good_dak_command {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            deb::control::dak::Command::from_reader(&mut file).unwrap();
        }
    };
}

macro_rules! test_bad_dak_command {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(deb::control::dak::Command::from_reader(&mut file).is_err());
        }
    };
}
macro_rules! test_good_dak_command_async {
    ($name:ident, $bytes:expr) => {
        // no async stubs yet
    };
}

macro_rules! test_bad_dak_command_async {
    ($name:ident, $bytes:expr) => {
        // no async stubs yet
    };
}
pub(crate) use test_bad_dak_command;
pub(crate) use test_bad_dak_command_async;
pub(crate) use test_good_dak_command;
pub(crate) use test_good_dak_command_async;

// .dsc

macro_rules! test_good_dsc {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::Dsc = deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}

macro_rules! test_bad_dsc {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::package::BinaryControl, _>(&mut file)
                    .is_err()
            );
        }
    };
}
macro_rules! test_good_dsc_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::Dsc = deb::control::de::from_reader_async(&mut file)
                .await
                .unwrap();
        }
    };
}

macro_rules! test_bad_dsc_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::package::BinaryControl, _>(
                    &mut file
                )
                .await
                .is_err()
            );
        }
    };
}
pub(crate) use test_bad_dsc;
pub(crate) use test_bad_dsc_async;
pub(crate) use test_good_dsc;
pub(crate) use test_good_dsc_async;

// .changes

macro_rules! test_good_changes {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::Changes =
                deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}

macro_rules! test_bad_changes {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::package::Changes, _>(&mut file)
                    .is_err()
            );
        }
    };
}

macro_rules! test_good_changes_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::Changes = deb::control::de::from_reader_async(&mut file)
                .await
                .unwrap();
        }
    };
}

macro_rules! test_bad_changes_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::package::Changes, _>(&mut file)
                    .await
                    .is_err()
            );
        }
    };
}

pub(crate) use test_bad_changes;
pub(crate) use test_bad_changes_async;
pub(crate) use test_good_changes;
pub(crate) use test_good_changes_async;

// DEBIAN/control

macro_rules! test_good_binarycontrol {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::BinaryControl =
                deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}
macro_rules! test_bad_binarycontrol {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::package::BinaryControl, _>(&mut file)
                    .is_err()
            );
        }
    };
}
macro_rules! test_good_binarycontrol_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::package::BinaryControl =
                deb::control::de::from_reader_async(&mut file)
                    .await
                    .unwrap();
        }
    };
}
macro_rules! test_bad_binarycontrol_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::package::BinaryControl, _>(
                    &mut file
                )
                .await
                .is_err()
            );
        }
    };
}
pub(crate) use test_bad_binarycontrol;
pub(crate) use test_bad_binarycontrol_async;
pub(crate) use test_good_binarycontrol;
pub(crate) use test_good_binarycontrol_async;

// Release

macro_rules! test_good_archive_release {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::archive::Release =
                deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}
macro_rules! test_bad_archive_release {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::archive::Release, _>(&mut file)
                    .is_err()
            );
        }
    };
}
macro_rules! test_good_archive_release_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::archive::Release = deb::control::de::from_reader_async(&mut file)
                .await
                .unwrap();
        }
    };
}
macro_rules! test_bad_archive_release_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::archive::Release, _>(&mut file)
                    .await
                    .is_err()
            );
        }
    };
}
pub(crate) use test_bad_archive_release;
pub(crate) use test_bad_archive_release_async;
pub(crate) use test_good_archive_release;
pub(crate) use test_good_archive_release_async;

// Package

macro_rules! test_good_archive_package {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            for package in
                deb::control::de::from_reader_iter::<deb::control::archive::Package, _>(&mut file)
            {
                let _package = package.unwrap();
            }
        }
    };
}
macro_rules! test_bad_archive_package {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            for package in
                deb::control::de::from_reader_iter::<deb::control::archive::Package, _>(&mut file)
            {
                if package.is_err() {
                    return;
                }
            }
            panic!("didn't find an error");
        }
    };
}
macro_rules! test_good_archive_package_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let mut package_iter = deb::control::de::from_reader_async_iter::<
                deb::control::archive::Package,
                _,
            >(&mut file);

            while let Some(package) = package_iter.next().await {
                let _package = package.unwrap();
            }
        }
    };
}
macro_rules! test_bad_archive_package_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let mut package_iter = deb::control::de::from_reader_async_iter::<
                deb::control::archive::Package,
                _,
            >(&mut file);

            while let Some(package) = package_iter.next().await {
                if package.is_err() {
                    return;
                }
            }
            panic!("didn't find an error");
        }
    };
}
pub(crate) use test_bad_archive_package;
pub(crate) use test_bad_archive_package_async;
pub(crate) use test_good_archive_package;
pub(crate) use test_good_archive_package_async;

// Apt

macro_rules! test_good_apt_source {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::apt::SourcesList =
                deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}
macro_rules! test_bad_apt_source {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::apt::SourcesList, _>(&mut file)
                    .is_err()
            );
        }
    };
}
macro_rules! test_good_apt_source_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::apt::SourcesList = deb::control::de::from_reader_async(&mut file)
                .await
                .unwrap();
        }
    };
}
macro_rules! test_bad_apt_source_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::apt::SourcesList, _>(&mut file)
                    .await
                    .is_err()
            );
        }
    };
}
pub(crate) use test_bad_apt_source;
pub(crate) use test_bad_apt_source_async;
pub(crate) use test_good_apt_source;
pub(crate) use test_good_apt_source_async;

// queued

macro_rules! test_good_queued_command {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::queued::Command =
                deb::control::de::from_reader(&mut file).unwrap();
        }
    };
}
macro_rules! test_bad_queued_command {
    ($name:ident, $bytes:expr) => {
        #[cfg(feature = "serde")]
        #[test]
        fn $name() {
            use std::io::{BufReader, Cursor};
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader::<deb::control::queued::Command, _>(&mut file)
                    .is_err()
            );
        }
    };
}
macro_rules! test_good_queued_command_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            let _: deb::control::queued::Command = deb::control::de::from_reader_async(&mut file)
                .await
                .unwrap();
        }
    };
}
macro_rules! test_bad_queued_command_async {
    ($name:ident, $bytes:expr) => {
        #[cfg(all(feature = "serde", feature = "tokio"))]
        #[tokio::test]
        async fn $name() {
            use std::io::Cursor;
            use tokio::io::BufReader;
            let mut file = BufReader::new(Cursor::new($bytes));
            assert!(
                deb::control::de::from_reader_async::<deb::control::queued::Command, _>(&mut file)
                    .await
                    .is_err()
            );
        }
    };
}
pub(crate) use test_bad_queued_command;
pub(crate) use test_bad_queued_command_async;
pub(crate) use test_good_queued_command;
pub(crate) use test_good_queued_command_async;
