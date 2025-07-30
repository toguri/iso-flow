-- NBA全30チームのマスターデータ投入
INSERT INTO teams (id, name, abbreviation, city, conference, division) VALUES
-- Eastern Conference - Atlantic Division
('BOS', 'Celtics', 'BOS', 'Boston', 'Eastern', 'Atlantic'),
('BKN', 'Nets', 'BKN', 'Brooklyn', 'Eastern', 'Atlantic'), 
('NYK', 'Knicks', 'NYK', 'New York', 'Eastern', 'Atlantic'),
('PHI', '76ers', 'PHI', 'Philadelphia', 'Eastern', 'Atlantic'),
('TOR', 'Raptors', 'TOR', 'Toronto', 'Eastern', 'Atlantic'),

-- Eastern Conference - Central Division
('CHI', 'Bulls', 'CHI', 'Chicago', 'Eastern', 'Central'),
('CLE', 'Cavaliers', 'CLE', 'Cleveland', 'Eastern', 'Central'),
('DET', 'Pistons', 'DET', 'Detroit', 'Eastern', 'Central'),
('IND', 'Pacers', 'IND', 'Indiana', 'Eastern', 'Central'),
('MIL', 'Bucks', 'MIL', 'Milwaukee', 'Eastern', 'Central'),

-- Eastern Conference - Southeast Division
('ATL', 'Hawks', 'ATL', 'Atlanta', 'Eastern', 'Southeast'),
('CHA', 'Hornets', 'CHA', 'Charlotte', 'Eastern', 'Southeast'),
('MIA', 'Heat', 'MIA', 'Miami', 'Eastern', 'Southeast'),
('ORL', 'Magic', 'ORL', 'Orlando', 'Eastern', 'Southeast'),
('WAS', 'Wizards', 'WAS', 'Washington', 'Eastern', 'Southeast'),

-- Western Conference - Northwest Division  
('DEN', 'Nuggets', 'DEN', 'Denver', 'Western', 'Northwest'),
('MIN', 'Timberwolves', 'MIN', 'Minnesota', 'Western', 'Northwest'),
('OKC', 'Thunder', 'OKC', 'Oklahoma City', 'Western', 'Northwest'),
('POR', 'Trail Blazers', 'POR', 'Portland', 'Western', 'Northwest'),
('UTA', 'Jazz', 'UTA', 'Utah', 'Western', 'Northwest'),

-- Western Conference - Pacific Division
('GSW', 'Warriors', 'GSW', 'Golden State', 'Western', 'Pacific'),
('LAC', 'Clippers', 'LAC', 'Los Angeles', 'Western', 'Pacific'),
('LAL', 'Lakers', 'LAL', 'Los Angeles', 'Western', 'Pacific'), 
('PHX', 'Suns', 'PHX', 'Phoenix', 'Western', 'Pacific'),
('SAC', 'Kings', 'SAC', 'Sacramento', 'Western', 'Pacific'),

-- Western Conference - Southwest Division
('DAL', 'Mavericks', 'DAL', 'Dallas', 'Western', 'Southwest'),
('HOU', 'Rockets', 'HOU', 'Houston', 'Western', 'Southwest'),
('MEM', 'Grizzlies', 'MEM', 'Memphis', 'Western', 'Southwest'),
('NOP', 'Pelicans', 'NOP', 'New Orleans', 'Western', 'Southwest'),
('SAS', 'Spurs', 'SAS', 'San Antonio', 'Western', 'Southwest')
ON CONFLICT (id) DO NOTHING;