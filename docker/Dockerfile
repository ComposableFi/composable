FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Composable. Here we create the binary."

ARG PROFILE=release
WORKDIR /composable

COPY . /composable

RUN pwd && ls && cd node && cargo build --$PROFILE

# ===== SECOND STAGE ======

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

USER composable
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/node_template"]
