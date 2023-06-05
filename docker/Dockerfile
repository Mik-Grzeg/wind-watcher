# syntax=docker/dockerfile:1.4
FROM rust:1.67 AS builder
ARG SERVICE_NAME
ARG BINARY_NAME
ARG BINARY_PATH

RUN apt-get update && apt-get install --yes libpq-dev ca-certificates 

WORKDIR ${SERVICE_NAME}

ADD . ./
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/${SERVICE_NAME}/target \
  cargo build --release --bin ${BINARY_NAME} && \
  mv target/release/${BINARY_NAME} /root

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

COPY --chown=${APP_USER}:${APP_USER} --from=builder /root/${BINARY_NAME} ${APP}/
COPY migrations/* ${APP}/migrations/

USER ${APP_USER}
WORKDIR ${APP}
