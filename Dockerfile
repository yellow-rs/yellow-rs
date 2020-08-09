ARG app_name=yellow-rs

FROM rust:1.45-alpine
WORKDIR /usr/src/$yellow-rs
RUN apk update && apk add --no-cache musl-dev  pkgconfig cairo-dev libressl-dev libffi-dev
COPY . /home/
RUN cargo build --release --manifest-path /home/Cargo.toml
