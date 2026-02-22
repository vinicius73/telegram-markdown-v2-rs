fix:
    cargo fmt --all
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    cargo test --workspace --all-features --locked

test:
    cargo test --workspace --all-features --locked

build:
    cargo build --workspace --all-features --locked

publish:
    cargo publish --locked