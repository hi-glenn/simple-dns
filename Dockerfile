# Use Ubuntu 24.04 as the base image
FROM ubuntu:24.04

# Set non-interactive frontend for package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies required for building Rust projects
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /build

COPY . .

ENV RUSTFLAGS="-C linker-features=-lld"
RUN cargo build
CMD ["cargo", "test"]