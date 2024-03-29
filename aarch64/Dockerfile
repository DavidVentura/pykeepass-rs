FROM quay.io/pypa/manylinux2014_aarch64 AS manylinux

FROM ubuntu:20.04 AS toolchain

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    automake \
    bison \
    bzip2 \
    ca-certificates \
    cmake \
    curl \
    file \
    flex \
    g++ \
    gawk \
    gdb \
    git \
    gperf \
    help2man \
    libncurses-dev \
    libssl-dev \
    libtool-bin \
    make \
    ninja-build \
    patch \
    pkg-config \
    python3 \
    sudo \
    texinfo \
    unzip \
    wget \
    xz-utils \
    libssl-dev \
    libffi-dev

# Install crosstool-ng
RUN curl -Lf https://github.com/crosstool-ng/crosstool-ng/archive/02d1503f6769be4ad8058b393d4245febced459f.tar.gz | tar xzf - && \
    cd crosstool-ng-02d1503f6769be4ad8058b393d4245febced459f && \
    ./bootstrap && \
    ./configure --prefix=/usr/local && \
    make -j4 && \
    make install && \
    cd .. && rm -rf crosstool-ng-*

COPY .config /tmp/toolchain.config

# Build cross compiler
RUN mkdir build && \
    cd build && \
    cp /tmp/toolchain.config .config && \
    export CT_ALLOW_BUILD_AS_ROOT_SURE=1 && \
    ct-ng build.2 || tail -n 500 build.log && \
    cd .. && \
    rm -rf build

FROM ubuntu:20.04

# Copy cross toolchain
COPY --from=toolchain /usr/aarch64-unknown-linux-gnu /usr/aarch64-unknown-linux-gnu

ENV DEBIAN_FRONTEND noninteractive
ENV PATH=$PATH:/usr/aarch64-unknown-linux-gnu/bin

ENV CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc \
    AR_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-ar \
    CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++

ENV TARGET_CC=aarch64-unknown-linux-gnu-gcc \
    TARGET_AR=aarch64-unknown-linux-gnu-ar \
    TARGET_RANLIB=aarch64-unknown-linux-gnu-ranlib \
    TARGET_CXX=aarch64-unknown-linux-gnu-g++ \
    TARGET_READELF=aarch64-unknown-linux-gnu-readelf \
    TARGET_SYSROOT=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/ \
    TARGET_C_INCLUDE_PATH=/usr/aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu/sysroot/usr/include/

ENV CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc


RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    curl \
    git \
    g++ \
    make \
    sudo \
    wget \
    software-properties-common \
    cmake \
    llvm-dev \
    libclang-dev \
    clang

# Target openssl & libffi
RUN export CC=$TARGET_CC && \
    export AR=$TARGET_AR && \
    export RANLIB=$TARGET_RANLIB && \
    echo "Building zlib" && \
    VERS=1.2.12 && \
    cd /tmp && \
    curl -sqLO https://zlib.net/zlib-$VERS.tar.gz && \
    tar xzf zlib-$VERS.tar.gz && cd zlib-$VERS && \
    CFLAGS="-O3 -fPIC" ./configure --prefix=/usr/aarch64-unknown-linux-gnu/ && \
    make -j4 && make -j4 install && \
    cd .. && rm -rf zlib-$VERS.tar.gz zlib-$VERS && \
    echo "Building OpenSSL" && \
    VERS=1.1.1o && \
    curl -sqO https://www.openssl.org/source/openssl-$VERS.tar.gz && \
    tar xzf openssl-$VERS.tar.gz && cd openssl-$VERS && \
    ./Configure linux-generic32 -fPIC --prefix=/usr/aarch64-unknown-linux-gnu/ && \
    make -j4 && make -j4 install_sw install_ssldirs && \
    cd .. && rm -rf openssl-$VERS.tar.gz openssl-$VERS && \
    echo "Building libffi" && \
    VERS=3.4.2 && \
    curl -sqLO https://github.com/libffi/libffi/releases/download/v$VERS/libffi-$VERS.tar.gz && \
    tar xzf libffi-$VERS.tar.gz && cd libffi-$VERS && \
    ./configure --prefix=/usr/aarch64-unknown-linux-gnu/ --disable-docs --host=aarch64-unknown-linux-gnu --build=$(uname -m)-linux-gnu && \
    make -j4 && make -j4 install && \
    cd .. && rm -rf libffi-$VERS.tar.gz libffi-$VERS
ENV AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_DIR=/usr/aarch64-unknown-linux-gnu/
RUN add-apt-repository -y ppa:deadsnakes/ppa && \
    apt-get update && \
    apt-get install -y \
    python3.5 python3.5-venv python3.5-dev \
    python3 python3-venv python3-dev python-is-python3

COPY --from=manylinux /opt/_internal /opt/_internal
COPY --from=manylinux /opt/python /opt/python

RUN curl -sS https://bootstrap.pypa.io/pip/3.5/get-pip.py | python3.5 && \
    python3.5 -m pip --version && \
    python3.5 -m pip install --no-cache-dir auditwheel build && \
    python3.5 -m pip install --no-cache-dir maturin auditwheel-symbols patchelf

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly
RUN rustup target add aarch64-unknown-linux-gnu --toolchain=nightly
RUN rustup default nightly
# https://docs.rs/aes/0.7.5/aes/
ENV RUSTFLAGS='-C target-feature=+aes -C link-arg=-s'
RUN cargo search --limit 0
WORKDIR /io
ENTRYPOINT ["maturin"]
