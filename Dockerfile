# Linux desktop build — Debian 13 (Trixie), same base as Raspberry Pi OS Trixie.
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

RUN cargo build --release --features linux-full