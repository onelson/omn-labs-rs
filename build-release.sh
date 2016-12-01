#!/bin/bash

set -e 

OS=$(uname -s)
ARCH=$(uname -p)
RELEASE="omn-labs-${OS,,}-${ARCH,,}"
cargo test
cargo build --release
mkdir -p dist/$RELEASE
cp target/release/omn-labs dist/$RELEASE/
cp -r assets dist/${RELEASE}/
cd dist && zip ${RELEASE}.zip -r ${RELEASE}