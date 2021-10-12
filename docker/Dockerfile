FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Composable. Here we create the binary."

RUN apt-get update
RUN apt-get -y install apt-transport-https ca-certificates curl gnupg2 software-properties-common
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
RUN add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable"


RUN apt-get update \  
&& apt-get install cron -y && apt-get install docker-ce vim -y

ARG VERSION=stable
ARG PROFILE=release
WORKDIR /composable

COPY . /composable

RUN pwd && ls && cd node && cargo build --$PROFILE



### STAGE 2 ###
FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Composable binary."
ARG PROFILE=release
COPY --from=builder /composable/target/$PROFILE/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /composable composable && \
	mkdir -p /composable/.local/share && \
	mkdir /data && \
	chown -R composable:composable /data && \
	chown -R composable:composable /composable/.local/share/local_testnet && \
	ln -s /data /composable/.local/share/local_testnet && \
	rm -rf /usr/bin /usr/sbin


# please update docker, toolchain toml and github action with toolchain at same time
# there is no parity docker with same version, any later version breaks 3.0 parity (need to use 4.0)
RUN rustup install ${VERSION}-x86_64-unknown-linux-gnu
RUN rustup default $VERSION
RUN rustup target add wasm32-unknown-unknown
# RUN pwd && ls && cd node && cargo build --$PROFILE

USER composable
EXPOSE 30333 9933 9944
VOLUME ["/data"]


CMD ["/usr/local/bin/node_template"]
