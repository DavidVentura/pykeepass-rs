set -euo pipefail
#docker build -t x86 -f Dockerfile_x86_64 .
#docker buildx build --platform linux/arm64/v8 -t att2 -f Dockerfile .
#docker build -t arm -f Dockerfile_arm .

echo 'Building x86'
docker run --env RUSTFLAGS='-C target-cpu=ivybridge -C link-arg=-s' -v ~/git/keepass-rs:/keepass-rs -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl

echo 'Building AArch64'
docker run --platform linux/arm64/v8 -v ~/git/keepass-rs:/keepass-rs -v ~/git/pykeepass-rs:/io att2 build --release --strip -i python3.5

echo 'Building armhf'
docker run --env RUSTFLAGS='-C target-feature=+v7,+neon -C linker=armv7-unknown-linux-gnueabihf-gcc -C link-arg=-s' \
        -v ~/git/keepass-rs:/keepass-rs \
	-v ~/git/pykeepass-rs:/io arm \
	build --release --strip -i python3.5 --target armv7-unknown-linux-musleabihf
bash whack_armv7.sh
