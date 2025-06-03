#!/bin/sh

cargo build --release
cp target/release/cli-file-compressor ./cli-file-compressor
chmod a+x cli-file-compressor
rm -rf target