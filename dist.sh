#!/usr/bin/env zsh

OSX_VERSION=`sw_vers --productversion`

mkdir -p dist
cargo build --target aarch64-apple-darwin
cargo build --target x86_64-apple-darwin

cp target/aarch64-apple-darwin/release/proxnox dist/proxnox-osx-aarch64
cp target/x86_64-apple-darwin/release/proxnox dist/proxnox-osx-x86_64

zip -m dist/proxnox-osx-aarch64.zip dist/proxnox-osx-aarch64
zip -m dist/proxnox-osx-x86_64.zip dist/proxnox-osx-x86_64

