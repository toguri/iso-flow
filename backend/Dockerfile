# Build stage
FROM rust:1.88-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this is cached if dependencies don't change)
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/scheduler.rs && \
    echo "fn main() {}" > src/bin/scrape.rs && \
    echo "fn main() {}" > src/bin/init_scrape.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY migrations_postgres ./migrations_postgres

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r app && useradd -r -g app app

# Copy built binary
COPY --from=builder /app/target/release/nba-trade-scraper /usr/local/bin/nba-trade-scraper

# Copy migrations
COPY --from=builder /app/migrations_postgres /app/migrations_postgres

# Set working directory
WORKDIR /app

# Change ownership
RUN chown -R app:app /app

# Switch to non-root user
USER app

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run the application
CMD ["nba-trade-scraper"]