FROM composablefi/rust 

WORKDIR  /usr/src/app

COPY . /usr/src/app

ENV TZ=Etc/UTC \
    USER=composable

# RUN cargo build --release

EXPOSE 8000

CMD ["cargo"]
