# PyKeepass-rs

Read-only interface for keepass databases, as exposed by [keepass-rs](https://github.com/sseemayer/keepass-rs).
Alternative to `pykeepass` because it is too slow to use on low-end devices (~9s to open my 134 entries database).

**This library is very alpha. I expect to break the interface constantly**


There is only one function: `get_all_groups_entries`:
```python
import pykeepass_rs
meta, groups, entries = pykeepass_rs.get_meta_and_entries("test.kdbx", password="somePassw0rd", keyfile=None)

for entry in entries:
    for e in entries:
        print(e.title, e.url)
```

Speed comparison on 150 entries:
```bash
# On my PC
PyKeepass       0m3,111s
PyKeepass-rs    0m0,089s

# On my phone
PyKeepass       0m8.56s
PyKeepass-rs    0m0.36s
```

# Note on PIP

It is **crucial** to upgrade pip or the wheels won't install!  
Python3.5 was dropped on pip==21, so you have to `pip3 install -U 'pip<21'`

# Building

x86 build:

```bash
docker build -t x86 -f Dockerfile_x86_64 .
docker run --env RUSTFLAGS='-C target-cpu=ivybridge -C link-arg=-s' -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl
```

ARM64 build:

```bash
docker buildx build --platform linux/arm64/v8 -t att2 -f Dockerfile .
docker run --platform linux/arm64/v8 -v ~/git/pykeepass-rs:/io att2 build --release --strip -i python3.5
```

ARMv7 build:

```bash
docker build -t arm -f Dockerfile_arm .
docker run --env RUSTFLAGS='-C target-feature=+v7,+neon -C linker=armv7-unknown-linux-gnueabihf-gcc -C link-arg=-s' \
	-v ~/git/pykeepass-rs:/io arm \
	build --release --strip -i python3.5 --target armv7-unknown-linux-musleabihf
# currently the .so in the wheel has incorrect name https://github.com/PyO3/maturin/issues/620
bash whack_armv7.sh
```


Release
```bash
unset DBUS_SESSION_BUS_ADDRESS
version=$(grep version Cargo.toml | head -1 | cut -d'"' -f2)
twine upload target/wheels/*$version*
```
# Issues

If you get 
```
#0 0.173 standard_init_linux.go:228: exec user process caused: exec format error
```
you need qemu-user-static:
```
sudo apt install -y qemu-user-static binfmt-support
```
