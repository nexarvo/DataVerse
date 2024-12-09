#!/bin/bash
set -x  # Enable command tracing
echo "Running migrations..."

# Ensure DATABASE_URL is available
if [ -z "$DATABASE_URL" ]; then
  echo "DATABASE_URL environment variable is not set."
  exit 1
fi

# Run migrations using sqlx
sqlx migrate run --database-url "$DATABASE_URL"

echo "Migrations completed."