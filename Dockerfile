# Desktop build for Debian 13 (Trixie) — supports amd64 and arm64 (aarch64).
# Raspberry Pi OS Trixie (64-bit) is a common arm64 target.
#
# Multi-arch usage (recommended):
#   docker buildx build --platform linux/amd64,linux/arm64 -t pi-smart-clock --push .
# Plain build (current host arch):
#   docker build -t pi-smart-clock .
FROM debian:trixie-slim

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        ca-certificates \
        curl \
        libsdl2-dev \
        libsdl2-ttf-dev \
        libsdl2-mixer-dev \
        fonts-dejavu-core \
        ffmpeg \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain stable --profile minimal \
    && rustup --version \
    && cargo --version

WORKDIR /src
COPY . .

# Use the single "full" desktop feature (the only supported target after Pico removal).
RUN cargo build --release --features full