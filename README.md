# PyKeepass-rs

Read-only interface for keepass databases, as exposed by [keepass-rs](https://github.com/sseemayer/keepass-rs).
Alternative to `pykeepass` because it is too slow to use on low-end devices (~9s to open my 134 entries database).


There is only one function: `get_all_groups_entries`:
```python
import pykeepass_rs
groups, entries = pykeepass_rs.get_all_groups_entries("test.kdbx", password="somePassw0rd", keyfile=None)
for e in entries
    print(e)
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
docker run -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl
```

ARM64 build:

```bash
docker buildx build --platform linux/arm64/v8 -t att2 -f Dockerfile .
docker run --platform linux/arm64/v8 -v ~/git/pykeepass-rs:/io arm build --release --strip -i python3.5
```

ARMv7 build:

```bash
~/maturin build --target armv7-unknown-linux-gnueabihf --release --strip
```


Release
```bash
twine upload target/wheels/*
```
