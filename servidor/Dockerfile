FROM rust:1.92 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p servidor

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/servidor .
CMD ["./servidor"]
