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
# Install full build essentials
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake build-essential && rm -rf /var/lib/apt/lists/*
# Copy entire backend folder to ensure fresh and correct build
COPY backend/ ./
# Perform a clean release build
RUN cargo build --release

# --- Final Runner ---
FROM debian:bookworm-slim
WORKDIR /app
# Runtime essentials including FFmpeg
RUN apt-get update && apt-get install -y ffmpeg openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the actual binary with a clear name
COPY --from=backend-builder /app/backend/target/release/wearos-video-backend /app/backend-server
# Copy frontend assets
COPY --from=frontend-builder /app/frontend/dist ./dist

# Final setup: Permissions and required directories
RUN chmod +x /app/backend-server && mkdir -p uploads static/previews

# Environment
ENV PORT=3000
EXPOSE 3000

# Diagnostic and Execution: check linked libraries then run
CMD ["sh", "-c", "ldd /app/backend-server && /app/backend-server"]
