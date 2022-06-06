FROM rust:1.61.0

RUN mkdir -p /usr/api
WORKDIR /usr/api
COPY . /usr/api

RUN rustc --version

RUN cargo install diesel_cli
RUN cargo install cargo-watch

RUN cargo build --release

