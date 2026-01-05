# syntax=docker/dockerfile:1

FROM oven/bun:1 AS base
WORKDIR /app

# Install deps first for better layer caching
COPY package.json bun.lockb* ./
RUN bun install --frozen-lockfile || bun install

# Copy source
COPY . .

# Runtime image
FROM oven/bun:1 AS runtime
WORKDIR /app

# Copy app from builder
COPY --from=base /app /app

# Default env
ENV NODE_ENV=production
ENV PORT=8787
ENV SQLITE_PATH=/data/sync.db

# Create data dir inside container (mounted by compose)
RUN mkdir -p /data

EXPOSE 8787

# Start the server (adjust if your entry differs)
CMD ["bun", "run", "src/index.ts"]

