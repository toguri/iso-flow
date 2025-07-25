-- PostgreSQL用のtrade_newsテーブル作成
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
);

-- インデックスの作成
CREATE INDEX IF NOT EXISTS idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_trade_news_team_id ON trade_news(team_id);
CREATE INDEX IF NOT EXISTS idx_trade_news_translation_status ON trade_news(translation_status);
CREATE INDEX IF NOT EXISTS idx_trade_news_link ON trade_news(link);

-- トリガーでupdated_atを自動更新
CREATE TRIGGER update_trade_news_updated_at BEFORE UPDATE
    ON trade_news FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();