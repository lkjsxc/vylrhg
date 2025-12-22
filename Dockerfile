FROM rust:alpine AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/vylrhg /usr/local/bin/vylrhg

ENTRYPOINT ["/usr/local/bin/vylrhg"]