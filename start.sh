#!/bin/bash

set -e

rm -rf target/debug || true
rm -rf target/release || true
cargo build --package template --features libtests
cargo test --package template --features libtests

PACKAGES=("a1" "a2" "b1" "b2" "c1" "c2" "d1" "d2" "e1" "e2" "f1" "f2" "g1" "g2" "h1" "h2")

for package in "${PACKAGES[@]}"; do
    cp src/template.rs src/$package.rs
done
