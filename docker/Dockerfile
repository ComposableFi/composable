FROM composablefi/rust:latest

ARG VERSION

USER root

RUN apt-get update -y && apt-get install wget curl -y --no-install-recommends \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /composable

COPY . /composable

LABEL description="Precompiled Docker image for Composable" \
      image.author="dayo@composable.finance, sre@composable.finance" \
      image.vendor="Composable Finance" \
      image.description="Composable is a hyper liquidity infrastructure layer for DeFi assets powered by Layer 2 Ethereum and Polkadot." \
      image.source="https://github.com/ComposableFi/composable/blob/main/docker/Dockerfile" \
      image.documentation="https://github.com/ComposableFi/composable#readme"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN curl "https://storage.googleapis.com/composable-binaries/community-releases/${VERSION}/composable-${VERSION}.tar.gz" | tar -xz 
RUN chmod +x target/release/composable && mv target/release/composable /usr/local/bin
RUN useradd -m -u 1001 -U -s /bin/sh -d /composable composable
RUN mkdir -p /composable/.local/share/local_testnet
RUN mkdir /data
RUN chown -R composable:composable /data
RUN chown -R composable:composable /composable/.local/share/local_testnet
RUN ln -s /data /composable/.local/share/local_testnet 

USER composable

EXPOSE 30333 9933 9944 9615

VOLUME ["/data"] 

CMD ["composable", "--version"]
