FROM rust:buster as base

WORKDIR /app

COPY ./src ./src
COPY Cargo.toml .
COPY ./gql/src ./gql/src
COPY ./gql/Cargo.toml ./gql/Cargo.toml
COPY ./rust_byc_helper/proc ./rust_byc_helper/proc
COPY ./rust_byc_helper/src ./rust_byc_helper/src
COPY ./rust_byc_helper/Cargo.toml ./rust_byc_helper/Cargo.toml
RUN cargo build -r -q

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates
RUN update-ca-certificates
COPY --from=base /app/target/release/listener .

ENV RUST_LOG="info, _=error"

ENTRYPOINT ["./listener"]
