#!/bin/bash

set -e

rm -rf target/debug || true
rm -rf target/release || true
cargo build
cargo test --features libtests

TARGET="problems"
DIRS=("z1")

mkdir -p $TARGET
for dir in "${DIRS[@]}"; do
    rsync -aP template $TARGET/$dir --delete
done
