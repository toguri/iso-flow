-- Create trade_news table
CREATE TABLE IF NOT EXISTS trade_news (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    external_id VARCHAR(255) UNIQUE NOT NULL,
    
    -- Original data
    title TEXT NOT NULL,
    description TEXT,
    
    -- Translation data (cache)
    title_ja TEXT,
    description_ja TEXT,
    translation_status VARCHAR(20) DEFAULT 'pending' CHECK (translation_status IN ('pending', 'completed', 'failed')),
    translated_at TIMESTAMP,
    
    -- Source information
    source_name VARCHAR(50) NOT NULL,
    source_url VARCHAR(500) NOT NULL,
    author VARCHAR(100),
    
    -- Classification
    category VARCHAR(20) NOT NULL CHECK (category IN ('Trade', 'Signing', 'Other')),
    is_official BOOLEAN DEFAULT FALSE,
    
    -- Timestamps
    published_at TIMESTAMP NOT NULL,
    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_trade_news_category ON trade_news(category);
CREATE INDEX IF NOT EXISTS idx_trade_news_source ON trade_news(source_name);
CREATE INDEX IF NOT EXISTS idx_trade_news_translation ON trade_news(translation_status);