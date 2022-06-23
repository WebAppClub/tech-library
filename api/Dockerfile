FROM rust:1.61.0 as builder

RUN cargo new --bin api
WORKDIR /api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# 必要になったらコメントアウトを外す
# COPY ./diesel.toml ./diesel.toml

RUN rustup component add rustfmt
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
# 必要になったらコメントアウトを外す
# COPY ./migrations ./migrations


RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM rust:1.61.0-bullseye

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /api .

RUN cargo install diesel_cli
RUN cargo install cargo-watch
