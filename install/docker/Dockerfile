FROM composablefi/rust 

WORKDIR  /usr/src/app

COPY . /usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=composable
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER 
RUN chown -R $APP_USER:$APP_USER /usr/src/app

RUN cargo build --release

EXPOSE 8000

