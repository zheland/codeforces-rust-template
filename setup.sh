#!/bin/bash

set -ex

(
    cd "$(dirname "$0")"
    rustup toolchain install 1.75.0-x86_64-unknown-linux-gnu
    rustup override set 1.75.0-x86_64-unknown-linux-gnu
)
