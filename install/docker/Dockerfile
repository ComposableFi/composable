FROM rust:1.55 as builder

ARG APP=/usr/src/app

USER $APP_USER
WORKDIR ${APP}

ENV TZ=Etc/UTC \
    APP_USER=composable

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata git acl packer \
    && apt-get install -y jq libssl-dev build-essential python3-pip \
    libclang-dev curl clang zip cmake curl pkg-config cmake ansible \
    && rm -rf /var/lib/apt/lists/*

RUN chown -R $APP_USER:$APP_USER ${APP}

COPY . ${APP}

RUN cargo build --release

EXPOSE 8000

