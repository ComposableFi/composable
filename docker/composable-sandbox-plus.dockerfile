FROM composablefi/ci-linux:2022-08-06 as builder

COPY . /build
WORKDIR /build

# NOTE: dirty hack until nix migration
RUN cargo +nightly build --release -p wasm-optimizer
RUN cargo +nightly build --release -p composable-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN cargo +nightly build --release -p picasso-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN cargo +nightly build --release -p dali-runtime-wasm --target wasm32-unknown-unknown --features=runtime-benchmarks
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/dali_runtime.wasm --output ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/picasso_runtime.wasm --output ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm
RUN ./target/release/wasm-optimizer --input ./target/wasm32-unknown-unknown/release/composable_runtime.wasm --output ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm

RUN export DALI_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/dali_runtime.optimized.wasm) && \
	export PICASSO_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/picasso_runtime.optimized.wasm) && \
	export COMPOSABLE_RUNTIME=$(realpath ./target/wasm32-unknown-unknown/release/composable_runtime.optimized.wasm) && \
	cargo build --release --features=builtin-wasm

# ===== SECOND STAGE ======

FROM composablefi/mmr-polkadot:latest as mmr-polkadot

FROM ubuntu:22.04
LABEL description="Docker image with Composable"

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
# TODO: basilisk obsolete -> remove
RUN groupadd -g 1000 service && useradd -m -s /bin/sh -g 1000 -G service service && \
	mkdir -p /apps/composable/scripts /apps/composable/target/release /apps/Basilisk-node/target/release /apps/polkadot/target/release && \
	apt-get update && apt-get install -y --no-install-recommends apt-utils ca-certificates curl git && \
	curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
	apt-get update && apt-get install -y --no-install-recommends nodejs && \
	npm install --global npm yarn && \
	curl https://github.com/galacticcouncil/Basilisk-node/releases/download/v7.0.1/basilisk -Lo /apps/Basilisk-node/target/release/basilisk && \
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
# ISSUE: it is old and seems not used - for sure some ports are not here anymore
EXPOSE 9945 9988 9998
ENTRYPOINT ["yarn", "composable_and_basilisk"]
