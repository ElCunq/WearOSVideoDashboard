# --- Frontend Builder ---
FROM node:20-alpine as frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Backend Builder ---
# Fixed to 1.85-bookworm to match runner and support edition 2024
FROM rust:1.85-bookworm as backend-builder
WORKDIR /app/backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY backend/src ./src
RUN cargo build --release

# --- Final Runner ---
# Using full bookworm to ensure all libraries (ssl, glibc) match the builder exactly
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ffmpeg openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=backend-builder /app/backend/target/release/wearos-video-backend ./server
COPY --from=frontend-builder /app/frontend/dist ./dist

# Create necessary runtime directories
RUN mkdir -p uploads static/previews

# Runtime Environment
ENV PORT=3000
EXPOSE 3000

# Execute with explicit path
CMD ["/app/server"]
