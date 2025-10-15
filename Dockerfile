# Get started with a build env with Rust nightly
FROM rust:1.90.0-bookworm AS builder

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang npm musl-tools musl-dev lld

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN npm install -g sass

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
RUN mkdir /app/acme
RUN mkdir /app/target-copy
WORKDIR /app

RUN --mount=type=bind,target=./rust-toolchain.toml,source=./rust-toolchain.toml \
    rustup toolchain install

RUN --mount=type=bind,target=./rust-toolchain.toml,source=./rust-toolchain.toml \
    rustup component add rust-src

ENV LEPTOS_HASH_FILES="true"

COPY . .

RUN printf '\nlib-cargo-args = ["--config", "./.cargo/wasm-extra.toml"]' >> ./Cargo.toml

# Build the app
# Speeding up: https://www.reddit.com/r/rust/comments/1irzbqf/cargo_build_cache_and_docker_cache_cause/
# Maybe use https://github.com/LukeMathWalker/cargo-chef ?
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/home/root/.cargo/registry \
    --mount=type=cache,target=/home/root/.cargo/cache \
    --mount=type=cache,target=/home/root/.cargo/src \
    --mount=type=cache,target=/home/root/.cargo/git/db/ \
    cargo leptos build --release -vv --bin-features ssl --bin-features ssr

RUN --mount=type=cache,target=/app/target \
    sh -c 'cp /app/target/release/luxalpa_com /app/target-copy/' && \
    sh -c 'cp /app/target/release/hash.txt /app/target-copy/' && \
    sh -c 'cp -R /app/target/site /app/target-copy/site'

FROM debian:bookworm-slim AS runner

# Copy the server binary to the /app directory
COPY --from=builder --chown=10001:10001 /app/target-copy/luxalpa_com /app/
COPY --from=builder --chown=10001:10001 /app/target-copy/hash.txt /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder --chown=10001:10001 /app/target-copy/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder --chown=10001:10001 /app/Cargo.toml /app/

# Fix permissions
COPY --from=builder --chown=10001:10001 /app/acme /app/acme

WORKDIR /app

USER 10001

# Set any required env variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_HASH_FILES="true"
EXPOSE 3000
EXPOSE 443
# Run the server
CMD ["/app/luxalpa_com"]