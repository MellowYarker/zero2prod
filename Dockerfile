# Builder stage for multi-stage build (faster!)
# This stage is discarded at the end of the build.
FROM rust:1.63.0 as builder

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working env to our Docker image
COPY . .

# Use metadata for query at compilation time.
ENV SQLX_OFFLINE true

# Let's build our binary!
# We'll use the release profile to make it fast
RUN cargo build --release

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
