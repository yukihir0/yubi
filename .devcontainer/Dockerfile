FROM rust:1.58.1

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

RUN rustup install nightly && \
    cargo install cargo-watch && \
    cargo install cargo-audit && \
    cargo install grcov
