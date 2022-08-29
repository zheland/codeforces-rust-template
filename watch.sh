#!/bin/bash

# Inotify does not work well on wsl.
# This script is used instead of inotifywait to work with WSL.
# Issue: https://github.com/microsoft/WSL/issues/4739

set -e

mkdir -p target/touch
mkdir -p target/problems

while true; do
    for ch in $( echo "⣼⣹⢻⠿⡟⣏⣧⣶" | sed -e 's/\(.\)/\1\n/g' ); do
        for bin_path in src/*.rs; do
            bin_name=$(basename -- "$bin_path" .rs)
            touch_path="target/touch/$bin_name"
            if [[ $touch_path -ot $bin_path ]]; then
                touch $touch_path
                cargo test --bin "$bin_name" || true
                size=$(stat --printf="%s" "$bin_path")
                if [ "$size" -gt "65536" ]; then
                    printf '%s size: \e[0;31m%+5s/65536\e[0m bytes\n' "$bin_path" "$size"
                elif [ "$size" -gt "60000" ]; then
                    printf '%s size: \e[0;33m%+5s/65536\e[0m bytes\n' "$bin_path" "$size"
                else
                    printf '%s size: \e[0;32m%+5s/65536\e[0m bytes\n' "$bin_path" "$size"
                fi
            fi
        done
        printf " %s\r" "$ch"
        sleep 0.1;
    done
done
