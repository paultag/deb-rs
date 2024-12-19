#!/bin/sh
set -e

OUTPUT_FILE=tests.rs


cat <<EOF
// Automatically generated from fixtures/rebuild.sh. Do not manually edit this
// file.

use super::common::*;

EOF

generate_tests() {
    MACRO=$1
    FIXTURE_PATH="$2"
    XTN="$3"
    for entry in $(find ${FIXTURE_PATH} -type f -name "*${XTN}"); do
        TEST_NAME=$(
          echo $entry |
              tr '-' '_' |
              tr '/' '_' |
              tr '+' '_' |
              tr '.' '_')

        cat <<EOF
$MACRO!($TEST_NAME, include_bytes!("$entry"));
${MACRO}_async!(async_${TEST_NAME}, include_bytes!("$entry"));
EOF
    done
}

# apt

generate_tests test_good_apt_source fixtures/unsigned/apt/sources .good
generate_tests test_bad_apt_source  fixtures/unsigned/apt/sources  .bad

# Dak

generate_tests test_good_dak_command  fixtures/unsigned/dak/command .good
generate_tests test_bad_dak_command   fixtures/unsigned/dak/command  .bad

# Packages

generate_tests test_good_dsc          fixtures/unsigned/package/dsc             .good
generate_tests test_bad_dsc           fixtures/unsigned/package/dsc              .bad

generate_tests test_good_changes      fixtures/unsigned/package/changes         .good
generate_tests test_bad_changes       fixtures/unsigned/package/changes          .bad

generate_tests test_good_binarycontrol fixtures/unsigned/package/binarycontrol  .good
generate_tests test_bad_binarycontrol  fixtures/unsigned/package/binarycontrol   .bad

# Archive

generate_tests test_good_archive_release fixtures/unsigned/archive/release .good
generate_tests test_bad_archive_release  fixtures/unsigned/archive/release  .bad

generate_tests test_good_archive_package fixtures/unsigned/archive/package .good
generate_tests test_bad_archive_package  fixtures/unsigned/archive/package  .bad

# queued

generate_tests test_good_queued_command fixtures/unsigned/queued .good
generate_tests test_bad_queued_command  fixtures/unsigned/queued  .bad
