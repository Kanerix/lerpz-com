#!/bin/bash
set -euo pipefail

# This script runs automatically on first Postgres startup via
# /docker-entrypoint-initdb.d/. It creates additional databases
# beyond the default POSTGRES_DB.
#
# Add database names to the array below as needed.

EXTRA_DATABASES=(
  "n8n"
)

for db in "${EXTRA_DATABASES[@]}"; do
  echo "Creating database: $db"
  psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-SQL
    SELECT 'CREATE DATABASE "$db"'
    WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$db')\gexec
SQL
  echo "Database '$db' created (or already exists)."
done