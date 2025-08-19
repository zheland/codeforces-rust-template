#!/bin/bash

set -e

PACKAGES=("a1" "a2" "b1" "b2" "c1" "c2" "d1" "d2" "e1" "e2" "f1" "f2" "g1" "g2" "h1" "h2")

for package in "${PACKAGES[@]}"; do
    cp src/tasks/template.rs src/tasks/$package.rs
    cargo build --bin $package
done

cargo test --bin template

bash watch.sh
