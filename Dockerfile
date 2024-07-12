FROM rust:latest as builder

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml ./Cargo.toml
COPY src ./src

RUN cargo add openssl --features vendored
RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip target/x86_64-unknown-linux-musl/release/github-stats

FROM alpine as runtime

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/github-stats /bin/github-stats

CMD ["/bin/github-stats"]