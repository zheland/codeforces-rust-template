#!/bin/bash

set -e

rm -rf target/debug || true
rm -rf target/release || true
bash build.sh

TARGET="online"
DIRS=("Z")

mkdir -p $TARGET
for dir in "${DIRS[@]}"; do
    rm -rf $TARGET/$dir
    rsync -aP . $TARGET/$dir --exclude='.git/' --exclude='target/rls/' --exclude='drafts/'
done

node clearlibs.js
