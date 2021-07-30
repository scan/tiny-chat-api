FROM rust:1.54 AS builder

RUN USER=root cargo new --bin tinychat-api
WORKDIR /tinychat-api
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release && \
    rm src/*.rs ./target/release/deps/tinychat_api*

ADD . ./

RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /tinychat-api/target/release/tinychat-api ${APP}/tinychat-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./tinychat-api"]