-- teamsテーブルにabbreviationカラムを追加
ALTER TABLE teams ADD COLUMN IF NOT EXISTS abbreviation TEXT;

-- 既存データの更新
UPDATE teams SET abbreviation = id WHERE abbreviation IS NULL;

-- NOT NULL制約を追加
ALTER TABLE teams ALTER COLUMN abbreviation SET NOT NULL;