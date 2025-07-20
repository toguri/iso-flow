# データベーススキーマ設計

## 概要
NBA Trade Scraperのデータベース設計ドキュメント。

## データベース選定
- **開発環境**: SQLite（シンプル、設定不要）
- **本番環境**: PostgreSQL（スケーラビリティ、JSON対応、全文検索）

## テーブル設計

### 1. teams（チーム情報）
```sql
CREATE TABLE teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,       -- 原文名
    name_ja VARCHAR(100),             -- カタカナ表記
    position VARCHAR(10),             -- PG, SG, SF, PF, C
    current_team_id INTEGER REFERENCES teams(id),
    jersey_number INTEGER,
    status VARCHAR(20) DEFAULT 'active',  -- active, injured, retired
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### 3. trade_news（トレードニュース）
```sql
CREATE TABLE trade_news (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    external_id VARCHAR(255) UNIQUE NOT NULL,  -- RSSフィードのID
    
    -- 原文データ
    title TEXT NOT NULL,
    description TEXT,
    
    -- 翻訳データ（キャッシュ）
    title_ja TEXT,
    description_ja TEXT,
    translation_status VARCHAR(20) DEFAULT 'pending', -- pending, completed, failed
    translated_at TIMESTAMP,
    
    -- ソース情報（引用元明記）
    source_name VARCHAR(50) NOT NULL,      -- ESPN, RealGM等
    source_url VARCHAR(500) NOT NULL,      -- 元記事URL
    author VARCHAR(100),                   -- 記者名（取得可能な場合）
    
    -- 分類情報
    category VARCHAR(20) NOT NULL,         -- Trade, Signing, Other
    is_official BOOLEAN DEFAULT FALSE,     -- 公式発表かどうか
    
    -- タイムスタンプ
    published_at TIMESTAMP NOT NULL,       -- 元記事の公開日時
    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- インデックス
CREATE INDEX idx_trade_news_published_at ON trade_news(published_at DESC);
CREATE INDEX idx_trade_news_category ON trade_news(category);
CREATE INDEX idx_trade_news_source ON trade_news(source_name);
CREATE INDEX idx_trade_news_translation ON trade_news(translation_status);
```

### 4. trade_news_teams（ニュースとチームの関連）
```sql
CREATE TABLE trade_news_teams (
    trade_news_id INTEGER REFERENCES trade_news(id) ON DELETE CASCADE,
    team_id INTEGER REFERENCES teams(id) ON DELETE CASCADE,
    PRIMARY KEY (trade_news_id, team_id)
);
```

### 5. trade_news_players（ニュースと選手の関連）
```sql
CREATE TABLE trade_news_players (
    trade_news_id INTEGER REFERENCES trade_news(id) ON DELETE CASCADE,
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    PRIMARY KEY (trade_news_id, player_id)
);
```

### 6. raw_feed_data（生データ保存）
```sql
CREATE TABLE raw_feed_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_name VARCHAR(50) NOT NULL,
    feed_url VARCHAR(500) NOT NULL,
    raw_content TEXT NOT NULL,         -- RSS XMLの生データ
    fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## データ保存・翻訳ポリシー

### 1. 元データの保持
- **原則**: 常に元の英語データを保存
- **理由**: データの完全性、翻訳品質向上時の再翻訳可能性

### 2. 翻訳タイミング
- **Option A**: スクレイピング後にバッチで翻訳（推奨）
  - メリット: API呼び出し効率化、表示高速化
  - デメリット: ストレージ使用量増加
  
- **Option B**: 表示時にリアルタイム翻訳
  - メリット: 常に最新の翻訳
  - デメリット: 表示遅延、API費用増加

### 3. 引用元の明記
画面表示時には必ず以下を表示：
- ソース名（ESPN、RealGM等）
- 元記事へのリンク
- 記事公開日時
- 可能な場合は記者名

例：
```
出典: ESPN | 2024-01-20 15:30
[元記事を見る]
```

## マイグレーション戦略

1. **開発環境（SQLite）**
   ```bash
   # sqliteファイルで管理
   backend/nba_trades.db
   ```

2. **本番移行時**
   - SQLiteからPostgreSQLへのデータ移行スクリプト
   - AUTOINCREMENT → SERIAL への変換

## 今後の拡張考慮事項
- 翻訳履歴テーブル（翻訳品質向上のため）
- ソース信頼度スコア
- ユーザーフィードバック機能