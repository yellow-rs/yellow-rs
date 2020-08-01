FROM rust:1.45

WORKDIR /home/aura/Workspace/Yellow-rs
COPY . .

RUN cargo install --path .

CMD ["cargo", "run", "--release"]
