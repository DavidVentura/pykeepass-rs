FROM python:3.5-alpine
RUN apk add curl wget tar bash libgcc gcc musl-dev
RUN wget https://github.com/PyO3/maturin/releases/download/v0.11.3/maturin-armv7-unknown-linux-musleabihf.tar.gz
RUN tar xvf maturin-armv7-unknown-linux-musleabihf.tar.gz
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly
RUN rustup target add aarch64-unknown-linux-musl --toolchain=nightly
RUN rustup default nightly
# https://docs.rs/aes/0.7.5/aes/
ENV RUSTFLAGS='-C target-feature=+aes -C link-arg=-s'
RUN cargo search --limit 0
WORKDIR /io
ENTRYPOINT ["/maturin"]
