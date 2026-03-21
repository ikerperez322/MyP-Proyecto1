FROM rust:1.92 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p cliente

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/cliente .
ENTRYPOINT ["./cliente"]
