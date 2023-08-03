FROM rust:latest as base

WORKDIR /app

COPY ./src ./src
COPY Cargo.toml .
COPY ./gql/src ./gql/src
COPY ./gql/Cargo.toml ./gql/Cargo.toml

RUN cargo build -r -q

FROM debian:11.7-slim

### needed for graphql
RUN apt-get update && apt-get install -y ca-certificates
RUN update-ca-certificates
###

COPY --from=base /app/target/release/listener .

ENV RUST_LOG="info, _=error"

ENTRYPOINT ["./listener"]
