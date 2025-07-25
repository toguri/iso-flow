#!/bin/bash
set -euo pipefail

# Migration script from SQLite to Aurora PostgreSQL
# This script handles the data migration from local SQLite to Aurora PostgreSQL

echo "=== SQLite to Aurora PostgreSQL Migration Script ==="
echo

# Check required environment variables
if [[ -z "${DATABASE_URL:-}" ]]; then
    echo "Error: DATABASE_URL environment variable is not set"
    echo "Please set the Aurora PostgreSQL connection URL"
    exit 1
fi

if [[ -z "${SQLITE_DB_PATH:-}" ]]; then
    SQLITE_DB_PATH="./nba_trades.db"
    echo "Using default SQLite database path: $SQLITE_DB_PATH"
fi

# Check if SQLite database exists
if [[ ! -f "$SQLITE_DB_PATH" ]]; then
    echo "Error: SQLite database not found at $SQLITE_DB_PATH"
    exit 1
fi

# Function to execute PostgreSQL commands
psql_exec() {
    psql "$DATABASE_URL" -c "$1"
}

# Function to check if PostgreSQL is accessible
check_postgres_connection() {
    echo "Checking PostgreSQL connection..."
    if psql "$DATABASE_URL" -c "SELECT 1" >/dev/null 2>&1; then
        echo "✓ Successfully connected to PostgreSQL"
    else
        echo "✗ Failed to connect to PostgreSQL"
        exit 1
    fi
}

# Function to create tables if they don't exist
create_tables() {
    echo
    echo "Creating tables in PostgreSQL..."
    
    # Create teams table
    psql_exec "
    CREATE TABLE IF NOT EXISTS teams (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        city TEXT NOT NULL,
        conference TEXT NOT NULL,
        division TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );"
    
    # Create trade_news table
    psql_exec "
    CREATE TABLE IF NOT EXISTS trade_news (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        link TEXT NOT NULL UNIQUE,
        published_at TIMESTAMP NOT NULL,
        author TEXT,
        category TEXT,
        source TEXT NOT NULL,
        team_id TEXT,
        description TEXT,
        scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        translated_title TEXT,
        translated_description TEXT,
        translation_status TEXT DEFAULT 'pending',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (team_id) REFERENCES teams(id)
    );"
    
    # Create indexes
    psql_exec "CREATE INDEX IF NOT EXISTS idx_trade_news_published_at ON trade_news(published_at DESC);"
    psql_exec "CREATE INDEX IF NOT EXISTS idx_trade_news_team_id ON trade_news(team_id);"
    psql_exec "CREATE INDEX IF NOT EXISTS idx_trade_news_translation_status ON trade_news(translation_status);"
    
    echo "✓ Tables created successfully"
}

# Function to export data from SQLite
export_sqlite_data() {
    echo
    echo "Exporting data from SQLite..."
    
    # Export teams data
    sqlite3 "$SQLITE_DB_PATH" <<EOF >/tmp/teams.csv
.headers on
.mode csv
.output /tmp/teams.csv
SELECT * FROM teams;
.quit
EOF
    
    # Export trade_news data
    sqlite3 "$SQLITE_DB_PATH" <<EOF >/tmp/trade_news.csv
.headers on
.mode csv
.output /tmp/trade_news.csv
SELECT * FROM trade_news;
.quit
EOF
    
    echo "✓ Data exported successfully"
}

# Function to import data to PostgreSQL
import_to_postgres() {
    echo
    echo "Importing data to PostgreSQL..."
    
    # Check if data already exists
    TEAM_COUNT=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM teams")
    NEWS_COUNT=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM trade_news")
    
    if [[ $TEAM_COUNT -gt 0 || $NEWS_COUNT -gt 0 ]]; then
        echo "Warning: Target database already contains data"
        echo "Teams: $TEAM_COUNT records"
        echo "Trade news: $NEWS_COUNT records"
        read -p "Do you want to continue? This will append data (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Migration cancelled"
            exit 0
        fi
    fi
    
    # Import teams data
    if [[ -f /tmp/teams.csv ]]; then
        echo "Importing teams..."
        psql "$DATABASE_URL" -c "\COPY teams FROM '/tmp/teams.csv' WITH CSV HEADER"
    fi
    
    # Import trade_news data
    if [[ -f /tmp/trade_news.csv ]]; then
        echo "Importing trade news..."
        psql "$DATABASE_URL" -c "\COPY trade_news FROM '/tmp/trade_news.csv' WITH CSV HEADER"
    fi
    
    echo "✓ Data imported successfully"
}

# Function to verify migration
verify_migration() {
    echo
    echo "Verifying migration..."
    
    # Count records in SQLite
    SQLITE_TEAMS=$(sqlite3 "$SQLITE_DB_PATH" "SELECT COUNT(*) FROM teams")
    SQLITE_NEWS=$(sqlite3 "$SQLITE_DB_PATH" "SELECT COUNT(*) FROM trade_news")
    
    # Count records in PostgreSQL
    PG_TEAMS=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM teams")
    PG_NEWS=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM trade_news")
    
    echo
    echo "Migration Summary:"
    echo "=================="
    echo "Teams:      SQLite: $SQLITE_TEAMS → PostgreSQL: $PG_TEAMS"
    echo "Trade News: SQLite: $SQLITE_NEWS → PostgreSQL: $PG_NEWS"
    echo
    
    # Cleanup temporary files
    rm -f /tmp/teams.csv /tmp/trade_news.csv
    
    echo "✓ Migration completed successfully!"
}

# Main migration process
main() {
    check_postgres_connection
    create_tables
    export_sqlite_data
    import_to_postgres
    verify_migration
}

# Run migration
main