# Stage 1: Build
FROM rust:1.79 as builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Needed dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/orbitask
COPY --from=builder /app/static /static
COPY Rocket.toml .  # If you’re using Rocket.toml for config
WORKDIR /data

EXPOSE 8000
CMD ["orbitask"]
