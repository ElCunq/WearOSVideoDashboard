# --- Frontend Builder ---
FROM node:20-alpine as frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Backend Builder ---
FROM rust:1.85-bookworm as backend-builder
WORKDIR /app/backend
# Build-time dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake && rm -rf /var/lib/apt/lists/*
COPY backend/Cargo.toml backend/Cargo.lock ./
# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY backend/src ./src
RUN cargo build --release

# --- Final Runner ---
FROM debian:bookworm-slim
WORKDIR /app
# Runtime essentials
RUN apt-get update && apt-get install -y ffmpeg openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy build artifacts with explicit names
COPY --from=backend-builder /app/backend/target/release/wearos-video-backend ./server
COPY --from=frontend-builder /app/frontend/dist ./dist

# Final preparations
RUN chmod +x ./server && mkdir -p uploads static/previews

ENV PORT=3000
EXPOSE 3000

# Use a shell wrapper to debug if it fails
CMD ["sh", "-c", "ls -l /app/server && /app/server"]
