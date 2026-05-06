# --- Frontend Builder ---
FROM node:20-alpine as frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Backend Builder ---
FROM rust:1.80-slim as backend-builder
WORKDIR /app/backend
# Install dependencies for FFmpeg and build tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev ffmpeg
COPY backend/Cargo.toml backend/Cargo.lock ./
# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY backend/src ./src
RUN cargo build --release

# --- Final Runner ---
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ffmpeg openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy built assets
COPY --from=backend-builder /app/backend/target/release/wearos-video-backend ./server
COPY --from=frontend-builder /app/frontend/dist ./dist

# Create necessary runtime directories
RUN mkdir -p uploads static/previews

# Environment
ENV PORT=3000
EXPOSE 3000

CMD ["./server"]
