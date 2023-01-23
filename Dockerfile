FROM rust:1.66.1-bullseye as builder

WORKDIR /opt/app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY src ./src
RUN cargo build --release

FROM debian:11-slim

RUN apt-get update && apt-get install ca-certificates openssl
COPY --from=builder /opt/app/target/release/recently-read-book /opt/app/

ENTRYPOINT ["/opt/app/recently-read-book"]