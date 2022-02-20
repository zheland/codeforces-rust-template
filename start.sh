#!/bin/bash

set -e

rm -rf target/debug || true
rm -rf target/release || true
bash build.sh

TARGET="online"
DIRS=("A" "B" "C" "D" "E" "F" "G" "H")

for dir in "${DIRS[@]}"; do
    if [[ -d "$TARGET/$dir" ]]; then
        echo "$TARGET/$dir exists."
        exit 1
    fi
done

mkdir -p $TARGET
for dir in "${DIRS[@]}"; do
    rsync -aP . $TARGET/$dir --exclude='.git/' --exclude='target/rls/'
done

node clearlibs.js
