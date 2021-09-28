#!/bin/bash
set -e
cargo build --release
mkdir -p target/rel
cp target/release/impossi-pong target/rel
cp -r assets target/rel
cp -r config target/rel
tar -czvf target/impossipong.tar.gz target/rel
rm -rf target/rel

cargo build --release --target x86_64-pc-windows-gnu
mkdir -p target/rel
cp target/x86_64-pc-windows-gnu/release/impossi-pong.exe target/rel
cp -r assets target/rel
cp -r config target/rel
zip -r target/impossipong.zip target/rel
rm -rf target/rel
