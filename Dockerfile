FROM rust:1.97 AS builder
WORKDIR /main
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:trixie
WORKDIR /main
COPY config.env /main/config.env
COPY --from=builder /main/target/release/rsgw /main/rsgw
RUN apt-get update && apt-get install -y ca-certificates
ENTRYPOINT ["/main/rsgw"]