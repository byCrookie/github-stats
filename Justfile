compose := "podman compose"

# ── Development ──────────────────────────────────────────────────────────────

build:
    cargo build

run:
    cargo run

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy -- -D warnings

test:
    cargo test

release:
    cargo build --release

clean:
    cargo clean

# ── Container ─────────────────────────────────────────────────────────────────

podman-build:
    {{compose}} build

podman-up:
    {{compose}} up -d

podman-down:
    {{compose}} down
