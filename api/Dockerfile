### develop environment ###
FROM rust:1.61.0 as develop
WORKDIR /api
RUN cargo install cargo-watch
COPY . .

### build ###
FROM develop as builder
RUN cargo build --release --bin api

### runtime ###
FROM debian:bullseye-slim as runtime
WORKDIR /api
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /api/target/release/api api
ENTRYPOINT [ "./api" ]
