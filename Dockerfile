# ── Stage 1: build ──────────────────────────────────────────────────────────
FROM rust:1-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Cache the Cargo registry so crates aren't re-downloaded on every build.
# The compiled dx binary is baked into the image layer as normal.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install dioxus-cli --version "=0.7.3" --locked

WORKDIR /build

COPY . .

# Cache the Cargo registry and build artifacts for incremental recompilation.
# Since dioxus 0.6, dx build outputs to target/dx/<crate>/release/web/public/
# instead of dist/. Because target/ is a cache mount (not persisted into the
# image layer), we copy the output to /build/dist within the same RUN command
# while the mount is still active.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    dx build --platform web --release && \
    cp -r /build/target/dx/my-file-cloud-client/release/web/public /build/dist

# ── Stage 2: serve ──────────────────────────────────────────────────────────
FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /build/dist /usr/share/nginx/html

EXPOSE 80
