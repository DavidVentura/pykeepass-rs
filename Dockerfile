FROM konstin2/maturin
RUN rustup target add aarch64-unknown-linux-gnu
WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]
