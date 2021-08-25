FROM python:3.5.10-slim-buster
RUN apt update && apt install -y build-essential curl wget tar
RUN wget https://github.com/PyO3/maturin/releases/download/v0.11.3/maturin-armv7-unknown-linux-musleabihf.tar.gz
RUN tar xvf maturin-armv7-unknown-linux-musleabihf.tar.gz
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTFLAGS='-C target-cpu=native'
WORKDIR /io
ENTRYPOINT ["/maturin"]
