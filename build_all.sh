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
docker buildx build --platform linux/arm64/v8 -t att2 -f Dockerfile .
docker run --platform linux/arm64/v8 -v ~/git/pykeepass-rs:/io att2 build --release --strip -i python3.5

echo 'Building x86'
docker build -t x86 -f Dockerfile_x86_64 .
docker run --env RUSTFLAGS='-C target-cpu=ivybridge' -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl --manylinux 2014
