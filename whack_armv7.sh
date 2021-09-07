#!/bin/bash
set -euo pipefail
tmp=$(mktemp -d)
here=$(pwd)
version=$(grep version Cargo.toml | head -1 | cut -d'"' -f 2)
wheel_path=$here/target/wheels/pykeepass_rs-$version-cp35-cp35m-manylinux_2_17_armv7l.manylinux2014_armv7l.whl
unzip "$wheel_path" -d $tmp
cd $tmp
mv pykeepass_rs/pykeepass_rs.cpython-35m-x86_64-linux-gnu.so pykeepass_rs/pykeepass_rs.cpython-35m-arm-linux-gnueabihf.so
sudo rm -i "$wheel_path"
sudo zip -r "$wheel_path" .
