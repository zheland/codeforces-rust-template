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
    for problem_path in problems/*; do
        problem_name=$(basename -- "$problem_path")
        file_path="$problem_path/src/$problem_name.rs"
        touch_path="target/touch/$problem_name"
        if [[ $touch_path -ot $file_path ]]; then
            touch $touch_path
            rustc $file_path -o target/problems/$problem_name \
            && cargo test --package "$problem_name" \
            || true
        fi
    done
    sleep 0.1;
done



exit

node importlibs.js
prev_modified=$( stat -c %Y "src/main.rs" )
echo "+ $args"
eval "$args" || true
while true; do
    last_modified=$( stat -c %Y "src/main.rs" )
    if [[ $prev_modified != $last_modified ]]; then
        printf "    \r"
        node importlibs.js
        last_modified=$( stat -c %Y "src/main.rs" )
        echo "+ $args"
        eval "$args" || true
        prev_modified=$last_modified
        sleep 1
    else
        for i in $( seq 1 12 ); do
            printf "$( printf $clocks | head -c $((i * 4)) | tail -c 4 )\r"
            sleep 0.02;
        done
    fi
done
