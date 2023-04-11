FROM rust:1.67 AS builder
ARG SERVICE_NAME
ARG BINARY_NAME
ARG BINARY_PATH

RUN cargo new --bin ${SERVICE_NAME} \
  && touch ${SERVICE_NAME}/src/lib.rs \
  && mkdir -p "${SERVICE_NAME}/src/${BINARY_PATH%/*}" \
  && cp ${SERVICE_NAME}/src/main.rs ${SERVICE_NAME}/src/${BINARY_PATH}

RUN apt-get update && apt-get install --yes libpq-dev ca-certificates 

WORKDIR ${SERVICE_NAME}

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release --bin ${BINARY_NAME}
RUN rm ./src/*.rs

ADD . ./
RUN cargo build --release --bin ${BINARY_NAME}
 
FROM debian:buster-slim

ARG SERVICE_NAME
ARG BINARY_NAME
ARG APP=/app

ENV APP_USER=appuser
RUN groupadd ${APP_USER} \
  && useradd -g ${APP_USER} ${APP_USER} \
  && mkdir -p ${APP} \
  && chown ${APP_USER}:${APP_USER} ${APP}
ENV PATH=$PATH:/${APP}

RUN apt-get update && apt-get install -y ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --chown=${APP_USER}:${APP_USER} --from=builder /${SERVICE_NAME}/target/release/${BINARY_NAME} ${APP}/
COPY migrations/* ${APP}/migrations/

USER ${APP_USER}
WORKDIR ${APP}
