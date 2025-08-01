-- 翻訳カラムの追加（既存のカラムがある場合は名前を変更）
ALTER TABLE trade_news 
    RENAME COLUMN translated_title TO title_ja;

ALTER TABLE trade_news 
    RENAME COLUMN translated_description TO description_ja;

-- translated_atカラムの追加
ALTER TABLE trade_news 
    ADD COLUMN IF NOT EXISTS translated_at TIMESTAMP;

-- translationカラムが存在しない場合のフォールバック
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'trade_news' 
                   AND column_name = 'title_ja') THEN
        ALTER TABLE trade_news ADD COLUMN title_ja TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'trade_news' 
                   AND column_name = 'description_ja') THEN
        ALTER TABLE trade_news ADD COLUMN description_ja TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name = 'trade_news' 
                   AND column_name = 'translation_status') THEN
        ALTER TABLE trade_news ADD COLUMN translation_status TEXT DEFAULT 'pending';
    END IF;
END $$;