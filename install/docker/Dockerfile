FROM rust:1.55 as builder

WORKDIR /usr/app/composable

# RUN cargo build --release

COPY . /usr/app/composable

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata git acl packer \
    && apt-get install -y jq libssl-dev build-essential jq python3-pip \
    libclang-dev curl clang zip cmake curl pkg-config cmake ansible \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=composable

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /usr/app/composable ${APP}/composable

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

EXPOSE 8000
