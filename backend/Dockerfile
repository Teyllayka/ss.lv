# 1. Use the official Rust image as the base
FROM rust:1.82

# 2. Install PostgreSQL client to use pg_isready
RUN apt-get update && \
    apt-get install -y postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# 3. Install sea-orm-cli using Cargo
RUN cargo install sea-orm-cli

# 4. Ensure Cargo's bin directory is in the PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

# 5. Set the working directory inside the container
WORKDIR /app

# 6. Copy the current directory contents into the container at /app
COPY ./ ./

# 7. Build the Rust application in release mode
RUN cargo build --release --jobs 3

# 8. Copy the entrypoint script into the container
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

# 9. Make the entrypoint script executable
RUN chmod +x /usr/local/bin/entrypoint.sh

# 10. Set the entrypoint to the script
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# 11. Define the default command to run the binary
CMD ["./target/release/backend"]
