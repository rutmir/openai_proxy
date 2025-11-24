 FROM rust:1.87-alpine AS builder

RUN apk update
#RUN apk add --no-cache libc-dev openssl-dev build-base musl-dev pkgconfig 
#RUN apk add --no-cache libc-dev libressl-dev build-base musl-dev pkgconfig
RUN apk add --no-cache musl-dev libressl-dev

# Install musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker caching for dependencies
COPY Cargo.toml ./

# Build dummy to cache dependencies
# RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir src

# Remove dummy file and copy actual source code
# RUN rm src/main.rs

# Copy source code and build the application
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release
# RUN cargo build --release --bin openai-proxy


FROM alpine:latest

#RUN apk update \
#    && apk add openssl ca-certificates

# Create a non-root user for security
RUN adduser -D appuser
USER appuser

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --chown=appuser:appuser --from=builder /app/target/x86_64-unknown-linux-musl/release/openai-proxy ./openai-proxy

# Expose port if your application is a server
# EXPOSE 3000

# Define the command to run your application
CMD ["./openai-proxy"]
