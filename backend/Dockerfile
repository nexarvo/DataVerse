FROM rust:1.68 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

CMD ["backend"]