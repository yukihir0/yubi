#!/bin/bash

set -eux

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
export TARGET_DIR="/tmp/target/debug"
export SOURCE_DIR="."
export OUTPUT_TYPE="html"
export OUTPUT_DIR="./coverage"

cargo +nightly build --verbose
cargo +nightly test --verbose

rm -rf $OUTPUT_DIR
grcov $TARGET_DIR -s $SOURCE_DIR --binary-path $TARGET_DIR -t $OUTPUT_TYPE --branch --llvm --ignore-not-existing -o $OUTPUT_DIR
