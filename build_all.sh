#!/bin/bash
set -euo pipefail

echo 'Building armhf'
docker build -t arm -f Dockerfile_arm .
docker run --env RUSTFLAGS='-C target-feature=+v7,+neon -C linker=armv7-unknown-linux-gnueabihf-gcc -C link-arg=-s' \
	-v ~/git/pykeepass-rs:/io arm \
	build --release --strip -i python3.5 --target armv7-unknown-linux-musleabihf \
	--manylinux 2014
bash whack_armv7.sh

echo 'Building AArch64'
(cd aarch64 && docker build -t cross-aarch64 .)
docker run -v ~/git/pykeepass-rs:/io --workdir /io cross-aarch64 build --release --strip -i python3.5 --target aarch64-unknown-linux-gnu --manylinux 2014
docker run -v ~/git/pykeepass-rs:/io --workdir /io cross-aarch64 build --release --strip -i python3.8 --target aarch64-unknown-linux-gnu --manylinux 2014

echo 'Building x86'
docker build -t x86 -f Dockerfile_x86_64 .
docker run --env RUSTFLAGS='-C target-cpu=ivybridge' -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl --manylinux 2014
docker run --env RUSTFLAGS='-C target-cpu=ivybridge' -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.8 --target x86_64-unknown-linux-musl --manylinux 2014
