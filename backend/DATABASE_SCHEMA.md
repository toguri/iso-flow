# データベーススキーマ設計

## 概要
NBA Trade Scraperのデータベース設計ドキュメント。

## データベース選定
- **開発環境**: PostgreSQL（本番環境と同一）
- **本番環境**: PostgreSQL（スケーラビリティ、JSON対応、全文検索）

## テーブル設計

### 1. teams（チーム情報）
```sql
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    code VARCHAR(3) UNIQUE NOT NULL,  -- 例: LAL, BOS
    name VARCHAR(100) NOT NULL,       -- 例: Los Angeles Lakers
    name_ja VARCHAR(100),             -- 例: ロサンゼルス・レイカーズ
    conference VARCHAR(10) NOT NULL,  -- East/West
    division VARCHAR(20) NOT NULL,    -- 例: Pacific, Atlantic
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 2. players（選手情報）
```sql
CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,       -- 例: LeBron James
    name_ja VARCHAR(200),             -- 例: レブロン・ジェームズ
    team_id INTEGER REFERENCES teams(id),
    position VARCHAR(10),             -- PG, SG, SF, PF, C
    jersey_number INTEGER,
    status VARCHAR(20),               -- active, injured, suspended
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 3. trade_news（トレード関連ニュース）
```sql
CREATE TABLE trade_news (
    id SERIAL PRIMARY KEY,
    external_id VARCHAR(500) UNIQUE NOT NULL,     -- RSS GUIDまたはURLのハッシュ
    title VARCHAR(500) NOT NULL,
    title_ja VARCHAR(500),                        -- 翻訳後タイトル
    description TEXT,
    description_ja TEXT,                          -- 翻訳後説明
    source_name VARCHAR(100) NOT NULL,            -- ESPN, RealGM等
    source_url TEXT NOT NULL,
    category VARCHAR(50),                         -- Trade, Rumor, Signing等
    is_official BOOLEAN DEFAULT FALSE,            -- 公式発表フラグ
    published_at TIMESTAMP NOT NULL,
    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    translation_status VARCHAR(20) DEFAULT 'pending', -- pending, completed, failed
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- インデックス
CREATE INDEX idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX idx_trade_news_category ON trade_news(category);
CREATE INDEX idx_trade_news_source ON trade_news(source_name);
CREATE INDEX idx_trade_news_translation_status ON trade_news(translation_status);
```

### 4. trade_details（トレード詳細）
将来的な拡張用
```sql
CREATE TABLE trade_details (
    id SERIAL PRIMARY KEY,
    news_id INTEGER REFERENCES trade_news(id),
    from_team_id INTEGER REFERENCES teams(id),
    to_team_id INTEGER REFERENCES teams(id),
    player_id INTEGER REFERENCES players(id),
    trade_type VARCHAR(50),  -- trade, waive, sign, draft
    contract_details JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 5. raw_feed_data（生フィードデータ）
デバッグ・監査用
```sql
CREATE TABLE raw_feed_data (
    id SERIAL PRIMARY KEY,
    feed_url TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    content_hash VARCHAR(64) UNIQUE NOT NULL,
    fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 命名規則

- テーブル名: 複数形のsnake_case（例: trade_news）
- カラム名: snake_case（例: published_at）
- 主キー: id
- 外部キー: {参照テーブル名}_id（例: team_id）
- タイムスタンプ: created_at, updated_at, {動作}_at

## インデックス戦略

1. **頻繁に検索される項目**
   - published_at（時系列表示）
   - category（カテゴリフィルタ）
   - source_name（ソース別表示）

2. **ユニーク制約**
   - external_id（重複防止）
   - content_hash（重複コンテンツ防止）

## GraphQL連携

- trade_newsテーブルがメインのデータソース
- リアルタイムフィードとDBキャッシュの併用
- 翻訳ステータスによる表示制御

例：
```
出典: ESPN | 2024-01-20 15:30
[元記事を見る]
```

## マイグレーション戦略

### 開発・本番環境共通（PostgreSQL）
```bash
# SQLx CLIを使用
sqlx migrate run
```

### マイグレーション管理
1. すべての環境でPostgreSQLを使用
2. migrations_postgresディレクトリで一元管理
3. バージョン管理とロールバックのサポート

## 今後の拡張考慮事項
- 翻訳履歴テーブル（翻訳品質向上のため）
- ソース信頼度スコア
- ユーザーフィードバック機能