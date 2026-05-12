FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip target/x86_64-unknown-linux-musl/release/github-stats

FROM scratch AS runtime
WORKDIR /bin/github-stats

ARG STATS_PORT=8080
ENV RUST_BACKTRACE=1

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/static ./static
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/github-stats ./github-stats

EXPOSE $STATS_PORT

ENTRYPOINT ["/bin/github-stats/github-stats"]