FROM rust:1.68 AS builder

WORKDIR /app

COPY backend/src/Cargo.toml backend/src/Cargo.lock ./
COPY backend/src/src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/your-binary-name /usr/local/bin/

CMD ["backend"]