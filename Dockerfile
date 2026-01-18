FROM rust:1.76-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/vylrhg /app/vylrhg
ENV DATA_DIR=/data
VOLUME ["/data"]
CMD ["/app/vylrhg"]
