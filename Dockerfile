FROM rust:1.45
WORKDIR .
COPY . .
RUN cargo install --path .
CMD ["cargo", "run", "--release"]
