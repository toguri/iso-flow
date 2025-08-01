-- translated_atカラムをTIMESTAMPTZに変更
ALTER TABLE trade_news 
    ALTER COLUMN translated_at TYPE TIMESTAMPTZ;

-- 既存のscraped_atカラムもTIMESTAMPTZに変更（一貫性のため）
ALTER TABLE trade_news 
    ALTER COLUMN scraped_at TYPE TIMESTAMPTZ;