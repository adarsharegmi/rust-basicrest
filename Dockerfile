# Use the Rust official image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to avoid build errors if there are no source files yet
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build your Rust project
RUN cargo build --release

# Copy the entire project to the container
COPY . .

# Set the entry point to your Rust application
CMD ["cargo", "run", "--release"]
