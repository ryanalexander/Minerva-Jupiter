# Build
FROM rustlang/rust:nightly-slim AS builder
USER 0:0
WORKDIR /home/rust/src

# Bundle
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Final
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --locked --release

CMD ["./target/release/jupiter"]