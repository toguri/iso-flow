-- Create teams table
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    abbreviation VARCHAR(3) NOT NULL,
    city VARCHAR(255) NOT NULL,
    conference VARCHAR(10) NOT NULL CHECK (conference IN ('East', 'West')),
    division VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create update_updated_at_column function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trade_news table  
CREATE TABLE IF NOT EXISTS trade_news (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    link TEXT NOT NULL UNIQUE,
    published_at TIMESTAMPTZ NOT NULL,
    author TEXT,
    category TEXT,
    source TEXT NOT NULL,
    team_id TEXT,
    description TEXT,
    scraped_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    translated_title TEXT,
    translated_description TEXT,
    translation_status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (team_id) REFERENCES teams(id)
);

-- Create index
CREATE INDEX IF NOT EXISTS idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_trade_news_team_id ON trade_news(team_id);
CREATE INDEX IF NOT EXISTS idx_trade_news_translation_status ON trade_news(translation_status);
CREATE INDEX IF NOT EXISTS idx_trade_news_link ON trade_news(link);

-- Create trigger
DROP TRIGGER IF EXISTS update_trade_news_updated_at ON trade_news;
CREATE TRIGGER update_trade_news_updated_at BEFORE UPDATE
    ON trade_news FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert teams data (optional for tests)
INSERT INTO teams (id, name, abbreviation, city, conference, division) VALUES
('BOS', 'Boston Celtics', 'BOS', 'Boston', 'East', 'Atlantic'),
('BKN', 'Brooklyn Nets', 'BKN', 'Brooklyn', 'East', 'Atlantic'),
('NYK', 'New York Knicks', 'NYK', 'New York', 'East', 'Atlantic'),
('PHI', 'Philadelphia 76ers', 'PHI', 'Philadelphia', 'East', 'Atlantic'),
('TOR', 'Toronto Raptors', 'TOR', 'Toronto', 'East', 'Atlantic'),
('CHI', 'Chicago Bulls', 'CHI', 'Chicago', 'East', 'Central'),
('CLE', 'Cleveland Cavaliers', 'CLE', 'Cleveland', 'East', 'Central'),
('DET', 'Detroit Pistons', 'DET', 'Detroit', 'East', 'Central'),
('IND', 'Indiana Pacers', 'IND', 'Indiana', 'East', 'Central'),
('MIL', 'Milwaukee Bucks', 'MIL', 'Milwaukee', 'East', 'Central'),
('ATL', 'Atlanta Hawks', 'ATL', 'Atlanta', 'East', 'Southeast'),
('CHA', 'Charlotte Hornets', 'CHA', 'Charlotte', 'East', 'Southeast'),
('MIA', 'Miami Heat', 'MIA', 'Miami', 'East', 'Southeast'),
('ORL', 'Orlando Magic', 'ORL', 'Orlando', 'East', 'Southeast'),
('WAS', 'Washington Wizards', 'WAS', 'Washington', 'East', 'Southeast'),
('DAL', 'Dallas Mavericks', 'DAL', 'Dallas', 'West', 'Southwest'),
('HOU', 'Houston Rockets', 'HOU', 'Houston', 'West', 'Southwest'),
('MEM', 'Memphis Grizzlies', 'MEM', 'Memphis', 'West', 'Southwest'),
('NOP', 'New Orleans Pelicans', 'NOP', 'New Orleans', 'West', 'Southwest'),
('SAS', 'San Antonio Spurs', 'SAS', 'San Antonio', 'West', 'Southwest'),
('DEN', 'Denver Nuggets', 'DEN', 'Denver', 'West', 'Northwest'),
('MIN', 'Minnesota Timberwolves', 'MIN', 'Minnesota', 'West', 'Northwest'),
('OKC', 'Oklahoma City Thunder', 'OKC', 'Oklahoma City', 'West', 'Northwest'),
('POR', 'Portland Trail Blazers', 'POR', 'Portland', 'West', 'Northwest'),
('UTA', 'Utah Jazz', 'UTA', 'Utah', 'West', 'Northwest'),
('GSW', 'Golden State Warriors', 'GSW', 'Golden State', 'West', 'Pacific'),
('LAC', 'Los Angeles Clippers', 'LAC', 'Los Angeles', 'West', 'Pacific'),
('LAL', 'Los Angeles Lakers', 'LAL', 'Los Angeles', 'West', 'Pacific'),
('PHX', 'Phoenix Suns', 'PHX', 'Phoenix', 'West', 'Pacific'),
('SAC', 'Sacramento Kings', 'SAC', 'Sacramento', 'West', 'Pacific')
ON CONFLICT DO NOTHING;