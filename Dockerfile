FROM rust:1.89-alpine

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

COPY ./src ./src
RUN cargo build --release

CMD ./target/release/sedna-worker