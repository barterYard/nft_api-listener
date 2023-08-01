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

RUN apt-get update && apt-get upgrade && apt-get install -y ca-certificates wget
# this is needed to run flow rust sdk (temp solution)
RUN wget https://snapshot.debian.org/archive/debian/20220507T034236Z/pool/main/o/openssl/libssl1.1_1.1.1o-1_amd64.deb
RUN dpkg -i *.deb
RUN update-ca-certificates

COPY --from=base /app/target/release/listener .

ENV RUST_LOG="info, _=error"

ENTRYPOINT ["./listener"]
