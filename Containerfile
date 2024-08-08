# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine AS builder

RUN apk update && apk add --no-cache --purge libc-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /work
COPY . .

RUN cargo fetch
RUN cargo build --release

########################################

FROM docker.io/alpine:latest AS runner

WORKDIR /app

COPY --from=builder /work/target/release/fav_sync_bot /app/

CMD ["/app/fav_sync_bot"]
