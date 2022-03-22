FROM debian:11-slim as builder
LABEL description="Docker image with Composable" 

ARG NIGHTLY_VERSION=nightly

ENV DEBIAN_FRONTEND=noninteractive

COPY . /build
WORKDIR /build

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils ca-certificates clang curl git libssl-dev llvm libudev-dev && \
	curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup default stable && \
	rustup update && \
	rustup update ${NIGHTLY_VERSION} && \
	rustup target add wasm32-unknown-unknown --toolchain ${NIGHTLY_VERSION} && \
	cargo build --release

# ===== SECOND STAGE ======

FROM composablefi/mmr-polkadot:latest as mmr-polkadot

FROM debian:11-slim

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN groupadd -g 1000 service && useradd -m -s /bin/sh -g 1000 -G service service && \
	mkdir -p /apps/composable/scripts /apps/composable/target/release /apps/Basilisk-node/target/release /apps/polkadot/target/release && \
	apt-get update && apt-get install -y --no-install-recommends apt-utils ca-certificates curl git && \
	curl -fsSL https://deb.nodesource.com/setup_17.x | bash - && \
	apt-get update && apt-get install -y --no-install-recommends nodejs && \
	npm install --global npm yarn && \
	curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v7.0.0/basilisk -Lo /apps/Basilisk-node/target/release/basilisk && \
	chmod +x /apps/Basilisk-node/target/release/basilisk && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete;

COPY --from=builder /build/target/release/composable /apps/composable/target/release/
COPY --from=mmr-polkadot /polkadot /apps/polkadot/target/release/
COPY ./scripts/polkadot-launch /apps/composable/scripts/polkadot-launch

WORKDIR /apps/composable/scripts/polkadot-launch

RUN chown -R service /apps/composable/scripts/polkadot-launch && \
	yarn && \
	sed -i 's/"--rpc-cors=all"/"--rpc-cors=all", "--ws-external", "--unsafe-rpc-external", "--rpc-methods=unsafe"/' composable_and_basilisk.json

USER service
EXPOSE 9945 9988 9998
ENTRYPOINT ["yarn", "composable_and_basilisk"]
