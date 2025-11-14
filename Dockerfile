# Stage 1: Build with dependency caching
FROM rust:bullseye AS builder

RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    pkg-config \
    nettle-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the dependency manifest. Cargo will generate a new lock file.
COPY Cargo.toml ./

# Fetch and cache the dependencies. This is the slow part that
# will only run again if Cargo.toml changes.
RUN cargo fetch

# Now copy your actual application source code.
COPY . .

# Build your application. This will be very fast because
# dependencies are already downloaded.
RUN cargo build --release

# Stage 2: Create the final, small image
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/ironshield-api /usr/local/bin/
EXPOSE 3000
CMD ["ironshield-api"]