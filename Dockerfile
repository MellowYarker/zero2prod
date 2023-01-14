FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as chef
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy all files from our working env to our Docker image
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project deps, not our app!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dep tree stays the same,
# all layers should be cached.
COPY . .

# Use metadata for query at compilation time.
ENV SQLX_OFFLINE true

# Let's build our binary!
# We'll use the release profile to make it fast
RUN cargo build --release --bin zero2prod

# Runtime stage (multi-stage).
# FROM rust:1.63.0 AS runtime
# This one is smaller.
# FROM rust:1.63.0-slim AS runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our deps.
# Install ca-certificates - it is needed to verify TLS certs
# when establishing HTTPS connections.
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environment.
COPY --from=builder /app/target/release/zero2prod zero2prod

# We need the configuration file at runtime!
COPY configuration configuration

# We want to use the production environment configuration.
# This sets the host to `0.0.0.0`.
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary.
ENTRYPOINT ["./zero2prod"]
