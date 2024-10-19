#!/bin/sh

# Exit immediately if a command exits with a non-zero status
set -e

# Function to wait for PostgreSQL to be ready
wait_for_db() {
  echo "Waiting for PostgreSQL to be available..."
  until pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER"; do
    echo "PostgreSQL is unavailable - sleeping"
    sleep 2
  done
  echo "PostgreSQL is available."
}

# Export PGPASSWORD for pg_isready to use
export PGPASSWORD="$DB_PASSWORD"

# Wait for the PostgreSQL database to be ready
wait_for_db

# Run database migrations using sea-orm-cli
echo "Running database migrations..."
sea-orm-cli migrate -d "$DATABASE_URL"

# Execute the main application
echo "Starting the Rust application..."
exec "$@"
