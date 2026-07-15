FROM rust:1.96 AS builder

ARG TARGETARCH

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools

RUN case "$TARGETARCH" in \
    amd64) echo "x86_64-unknown-linux-musl"   ;; \
    arm64) echo "aarch64-unknown-linux-musl"  ;; \
    *)     echo "Unsupported architecture: $TARGETARCH" >&2 && exit 1 ;; \
    esac > /rust_target.txt

RUN rustup target add "$(cat /rust_target.txt)"

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --target "$(cat /rust_target.txt)" --release --locked
RUN cp "target/$(cat /rust_target.txt)/release/github-stats" /app/github-stats
RUN strip /app/github-stats

FROM scratch AS runtime
WORKDIR /bin/github-stats

ARG STATS_PORT=8080

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/static ./static
COPY --from=builder /app/github-stats ./github-stats

EXPOSE $STATS_PORT

ENTRYPOINT ["/bin/github-stats/github-stats"]
