#!/bin/bash
set -e

echo "🚀 Starting test database..."
docker-compose -f docker-compose.test.yml down -v
docker-compose -f docker-compose.test.yml up -d

echo "⏳ Waiting for database to be ready..."
until docker exec iso-flow-test-db pg_isready -U test_user -d test_iso_flow; do
  echo "Database is unavailable - sleeping"
  sleep 1
done

echo "✅ Database is ready!"

echo "🧪 Running tests..."
cargo test -- --include-ignored

echo "📊 Running coverage..."
cargo tarpaulin --out Html --out Lcov --output-dir coverage

echo "🧹 Cleaning up..."
docker-compose -f docker-compose.test.yml down -v

echo "✨ Tests completed!"