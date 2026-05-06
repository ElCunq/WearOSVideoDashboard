# --- Frontend Builder ---
FROM node:20-alpine as frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Backend Builder ---
# Updated to latest to support edition 2024 features required by modern crates
FROM rust:latest as backend-builder
WORKDIR /app/backend
# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY backend/Cargo.toml backend/Cargo.lock ./
# Pre-build dependencies for caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY backend/src ./src
RUN cargo build --release

# --- Final Runner ---
FROM debian:bookworm-slim
WORKDIR /app
# Runtime dependencies including FFmpeg for video processing
RUN apt-get update && apt-get install -y ffmpeg openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy build artifacts
COPY --from=backend-builder /app/backend/target/release/wearos-video-backend ./server
COPY --from=frontend-builder /app/frontend/dist ./dist

# Create necessary runtime directories
RUN mkdir -p uploads static/previews

# Runtime Environment
ENV PORT=3000
EXPOSE 3000

CMD ["./server"]
