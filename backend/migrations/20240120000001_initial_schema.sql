-- Teams table
CREATE TABLE IF NOT EXISTS teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code VARCHAR(3) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    name_ja VARCHAR(100),
    conference VARCHAR(10) NOT NULL,
    division VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Players table
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    name_ja VARCHAR(100),
    position VARCHAR(10),
    current_team_id INTEGER REFERENCES teams(id),
    jersey_number INTEGER,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Trade news table
CREATE TABLE IF NOT EXISTS trade_news (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    external_id VARCHAR(255) UNIQUE NOT NULL,
    
    -- Original content
    title TEXT NOT NULL,
    description TEXT,
    
    -- Japanese translations (cached)
    title_ja TEXT,
    description_ja TEXT,
    translation_status VARCHAR(20) DEFAULT 'pending',
    translated_at TIMESTAMP,
    
    -- Source attribution
    source_name VARCHAR(50) NOT NULL,
    source_url VARCHAR(500) NOT NULL,
    author VARCHAR(100),
    
    -- Classification
    category VARCHAR(20) NOT NULL,
    is_official BOOLEAN DEFAULT FALSE,
    
    -- Timestamps
    published_at TIMESTAMP NOT NULL,
    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Trade news teams relationship
CREATE TABLE IF NOT EXISTS trade_news_teams (
    trade_news_id INTEGER REFERENCES trade_news(id) ON DELETE CASCADE,
    team_id INTEGER REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (trade_news_id, team_id)
);

-- Trade news players relationship
CREATE TABLE IF NOT EXISTS trade_news_players (
    trade_news_id INTEGER REFERENCES trade_news(id) ON DELETE CASCADE,
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (trade_news_id, player_id)
);

-- Raw feed data storage
CREATE TABLE IF NOT EXISTS raw_feed_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_name VARCHAR(50) NOT NULL,
    feed_url VARCHAR(500) NOT NULL,
    raw_content TEXT NOT NULL,
    fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_trade_news_category ON trade_news(category);
CREATE INDEX IF NOT EXISTS idx_trade_news_source ON trade_news(source_name);
CREATE INDEX IF NOT EXISTS idx_trade_news_translation ON trade_news(translation_status);