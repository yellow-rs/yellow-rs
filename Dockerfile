FROM rust:1.45-alpine
RUN apk update && apk add --no-cache musl-dev  pkgconfig cairo-dev libressl-dev libffi-dev
COPY . /home/
RUN cargo build --release --manifest-path /home/Cargo.toml
