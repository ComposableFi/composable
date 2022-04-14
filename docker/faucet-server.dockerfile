FROM composablefi/rust:latest

ARG VERSION

USER root

RUN apt-get update -y && apt-get install wget curl -y --no-install-recommends \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY . .

WORKDIR utils/faucet-server

LABEL description="Faucet-server is a utility library for Composable" \
      image.author="dayo@composable.finance, sre@composable.finance" \
      image.vendor="Composable Finance" \
      image.description="Composable is a hyper liquidity infrastructure layer for DeFi assets powered by Layer 2 Ethereum and Polkadot." \
      image.source="https://github.com/ComposableFi/composable/blob/main/docker/Dockerfile" \
      image.documentation="https://github.com/ComposableFi/composable#readme"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN rustup default nightly && rustup update

RUN cargo +nightly build --release -p faucet-server
    
RUN mv target/release/faucet-server /usr/local/bin && chmod +x /usr/local/bin/faucet-server  


EXPOSE 8088


CMD ["faucet-server", "--version"]
