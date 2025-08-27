# Rust geometric engine container
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-geometry-engine /usr/local/bin/rust-geometry-engine
EXPOSE 8080
CMD ["rust-geometry-engine"]
