# PyKeepass-rs

Read-only interface for keepass databases, as exposed by [keepass-rs](https://github.com/sseemayer/keepass-rs).
Alternative to `pykeepass` because it is too slow to use on low-end devices (~9s to open my 134 entries database).


There is only one function: `get_all_entries`:
```python
import pykeepass_rs
for e in pykeepass_rs.get_all_entries("test.kdbx", password="somePassw0rd", keyfile=None):
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

x86 build:

```bash
docker build -t x86 -f Dockerfile_x86_64 .
docker run -v ~/git/pykeepass-rs:/io x86 build --release --strip -i python3.5 --target x86_64-unknown-linux-musl
```

ARM build:

```bash
docker buildx build --platform linux/arm64/v8 -t att2 -f Dockerfile .
docker run -v ~/git/pykeepass-rs:/io arm build --release --strip -i python3.5
```


Release
```bash
twine upload target/wheels/*
```
