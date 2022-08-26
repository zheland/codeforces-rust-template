#!/bin/bash

# Inotify does not work well on wsl.
# This script is used instead of inotifywait to work with WSL.
# Issue: https://github.com/microsoft/WSL/issues/4739

set -e

clocks=""
clocks="$clocks\U0001f550\U0001f551\U0001f552\U0001f553\U0001f554\U0001f555"
clocks="$clocks\U0001f556\U0001f557\U0001f558\U0001f559\U0001f55a\U0001f55b"

mkdir -p target/touch
mkdir -p target/problems

while true; do
    for i in $( seq 1 12 ); do
        for bin_path in src/*.rs; do
            bin_name=$(basename -- "$bin_path" .rs)
            touch_path="target/touch/$bin_name"
            if [[ $touch_path -ot $bin_path ]]; then
                touch $touch_path
                cargo test --bin "$bin_name" || true
            fi
        done
        printf " $( printf $clocks | head -c $((i * 4)) | tail -c 4 )\r"
        sleep 0.1;
        printf "  \r"
    done
done
