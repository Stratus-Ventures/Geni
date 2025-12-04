# Latest stable Rust (new enough for edition 2024)
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy everything in (simple & predictable)
COPY . .

# Build in release mode so the first run is fast-ish
RUN cargo build --release

# Expose whatever port your Axum server uses (change if needed)
EXPOSE 8080

# Run the web server
CMD ["cargo", "run", "--release"]