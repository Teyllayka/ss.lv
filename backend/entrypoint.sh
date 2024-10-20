#!/usr/bin/env bash
set -e

# Function to check if PostgreSQL is ready
wait_for_postgres() {
  echo "Waiting for PostgreSQL to be ready..."
  while ! pg_isready -h "$POSTGRES_HOST" -U "$POSTGRES_USER" > /dev/null 2>&1; do
    sleep 2
  done
  echo "PostgreSQL is ready."
}

# Run the wait function
wait_for_postgres

# Run database migrations
echo "Running database migrations..."
sea-orm-cli migrate

# Execute the main container command
exec "$@"
