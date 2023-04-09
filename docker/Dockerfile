FROM rust:1.67 AS builder
ARG APP_NAME

RUN cargo new --bin ${APP_NAME} && touch ${APP_NAME}/src/lib.rs
RUN apt-get update && apt-get install --yes libpq-dev tree

WORKDIR ${APP_NAME}

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm ./src/*.rs

ADD . ./
RUN cargo build --release
 
FROM debian:buster-slim
ARG APP_NAME
ARG APP=/app

ENV APP_USER=appuser
RUN groupadd ${APP_USER} \
  && useradd -g ${APP_USER} ${APP_USER} \
  && mkdir -p ${APP}

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} ${APP}/${APP_NAME}
USER ${APP_USER}
WORKDIR ${APP}

CMD ./${APP_NAME}
