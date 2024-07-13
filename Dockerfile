FROM rust:latest
WORKDIR /app

COPY Cargo.toml ./Cargo.toml
COPY src ./src

RUN cargo build --release
COPY static ./target/release/static
ENTRYPOINT ["./target/release/github-stats"]