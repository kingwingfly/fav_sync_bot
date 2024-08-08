# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine as builder

RUN apk update && apk add --no-cache libc-dev openssl-dev openssl pkgconfig

WORKDIR /work
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release -vv

########################################

FROM docker.io/alpine:latest as runner

WORKDIR /app

COPY --from=builder /work/target/release/fav_sync_bot /app/

CMD ["/app/fav_sync_bot"]
