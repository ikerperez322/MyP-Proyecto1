FROM rust:1.92 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p servidor

FROM debian:bookworm-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/servidor .
CMD ["./servidor"]
