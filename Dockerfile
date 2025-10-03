# Use the official Rust image as the base image
FROM rust:1.84

# Install necessary packages
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    protobuf-compiler \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Install nightly toolchain + targets/components
RUN rustup toolchain install nightly && rustup default nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup component add rust-src --toolchain nightly

# Set the working directory
WORKDIR /usr/src/app

# Clone your fork of the SDK (with your custom pallet)
RUN git clone https://github.com/decenzio/polkadot-sdk.git --depth 1 --branch 1.0.2 --recurse-submodules

# Configure Cargo to use Git CLI (helps in some CI/network setups)
RUN mkdir -p ~/.cargo && echo "[net]\ngit-fetch-with-cli = true" > ~/.cargo/config
ENV CARGO_HOME=~/.cargo

# Build your binaries from your SDK
WORKDIR /usr/src/app/polkadot-sdk
RUN cargo build --release -p parachain-template-node
RUN cargo build --release -p polkadot

# Binaries folder for Zombienet + config
WORKDIR /usr/src/app/polkadot-sdk/binaries

# Zombienet
RUN wget https://github.com/paritytech/zombienet/releases/download/v1.3.128/zombienet-linux-x64 \
    && chmod +x zombienet-linux-x64

# Use the SDK-built polkadot (do NOT download a prebuilt one)
RUN ln -sf /usr/src/app/polkadot-sdk/target/release/polkadot ./polkadot

# Copy network configuration
COPY config.toml /usr/src/app/polkadot-sdk/binaries/config.toml

ENV BIND_INTERFACE=0.0.0.0

# Launch Zombienet
CMD ["./zombienet-linux-x64", "-p", "native", "-c", "1", "spawn", "config.toml"]
