FROM rust:1.85-slim AS builder
RUN apt-get update && apt-get install -y pkg-config build-essential cmake
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
EXPOSE 8443
COPY --from=builder /app/target/release/prod /usr/local/bin/monolith-backend
CMD ["monolith-backend"]
