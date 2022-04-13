# ===== Rust builder =====
FROM phusion/baseimage:focal-1.1.0 as builder
LABEL maintainer="Composable"

ARG RUST_TOOLCHAIN=nightly-2021-11-07
ARG PROFILE=release

ENV CARGO_HOME="/cargo-home"
ENV PATH="/cargo-home/bin:$PATH"

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

RUN apt-get update && \
    apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
    apt-get install -y cmake pkg-config libssl-dev git clang


RUN rustup toolchain uninstall $(rustup toolchain list) && \
    rustup toolchain install $RUST_TOOLCHAIN && \
    rustup default $RUST_TOOLCHAIN && \
    rustup target add wasm32-unknown-unknown --toolchain $RUST_TOOLCHAIN && \
    rustup target list --installed


####works inside repo folder###

RUN mkdir composable
WORKDIR /composable
COPY . .



RUN export PATH="$PATH:$HOME/.cargo/bin" && \
    export SKIP_WASM_BUILD=1 && \
    rustup show && \
    cargo build --release

RUN cd target/release && ls -la

# ===== RUN ======

FROM phusion/baseimage:focal-1.1.0

COPY --from=builder /composable/target/release/composable /usr/local/bin

EXPOSE 9844
EXPOSE 9833
EXPOSE 40333
EXPOSE 30333

VOLUME ["/chain-data"]

