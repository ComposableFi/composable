FROM composablefi/rust:latest

ARG VERSION=v0.1.0

USER root

RUN apt-get update -y && apt-get install wget curl -y --no-install-recommends \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /composable

LABEL description="Precompiled Docker image for Composable" \
      image.author="dayo@composable.finance, sre@composable.finance" \
      image.vendor="Composable Finance" \
      image.description="Composable is a hyper liquidity infrastructure layer for DeFi assets powered by Layer 2 Ethereum and Polkadot." \
      image.source="https://github.com/ComposableFi/composable/blob/main/docker/Dockerfile" \
      image.documentation="https://github.com/ComposableFi/composable#readme"

RUN curl "https://storage.googleapis.com/composable-binaries/releases/client/${VERSION}/composable" -o composable 

RUN chmod +x composable && mv composable /usr/local/bin

RUN composable --version

RUN useradd -m -u 1001 -U -s /bin/sh -d /composable composable && \
    mkdir -p /composable/.local/share/local_testnet && \
    mkdir /data && \
    chown -R composable:composable /data && \
    chown -R composable:composable /composable/.local/share/local_testnet && \
    ln -s /data /composable/.local/share/local_testnet 
USER composable

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"] 

CMD ["composable", "--version"]
