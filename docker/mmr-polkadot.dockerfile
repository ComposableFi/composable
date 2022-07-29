FROM debian:11-slim as builder

ARG NIGHTLY_VERSION=nightly

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils build-essential ca-certificates clang curl git libclang-dev libudev-dev llvm pkg-config cmake && \
	curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup default stable && \
	rustup update && \
	rustup update ${NIGHTLY_VERSION} && \
	rustup target add wasm32-unknown-unknown --toolchain ${NIGHTLY_VERSION} && \
	git clone -b mmr-polkadot-v0.9.24 https://github.com/composableFi/polkadot

WORKDIR /polkadot

RUN export PATH="$PATH:$HOME/.cargo/bin" && \
	cargo build --release

# ===== SECOND STAGE ======

FROM debian:11-slim

COPY --from=builder /polkadot/target/release/polkadot /
