FROM rust:1.56@sha256:17fdb4314fc0a2bb14e7d1e533cb11cc30b3373778d9cccd4e75fdc235b4148c as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/rust-webapp /usr/local/bin/rust-webapp
CMD ["rust-webapp"]
