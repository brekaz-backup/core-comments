FROM lukemathwalker/cargo-chef:latest AS chef
RUN apt update && apt install -y cmake capnproto libsasl2-dev protobuf-compiler libprotobuf-dev \
    libssl-dev libz-dev libclang-dev libsasl2-dev
WORKDIR /app

FROM chef AS planner
COPY ./schema ./schema
COPY ./Cargo.toml ./
COPY ./src ./src
COPY ./build.rs ./build.rs
COPY ./deps ./deps

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y cmake capnproto libsasl2-dev protobuf-compiler libprotobuf-dev \
    libsasl2-dev

WORKDIR /app
COPY . .
COPY --from=builder /app/target/release/blumer-ms-comments /usr/local/bin/app
COPY --from=builder   /app/private_key.pem /usr/local/bin/private_key.pem

COPY --from=builder /app/deps /usr/local/bin
COPY --from=builder /app/deps /usr/local/bin/deps
COPY --from=builder /app/deps /deps

ENTRYPOINT ["/usr/local/bin/app"]