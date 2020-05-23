FROM rust

COPY yellow-rs /bin/yellow-rs

CMD ["/bin/yellow-rs"]
