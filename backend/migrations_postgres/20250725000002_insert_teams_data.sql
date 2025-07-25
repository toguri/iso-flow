-- NBA全30チームのマスターデータ投入
INSERT INTO teams (id, name, city, conference, division) VALUES
-- Eastern Conference - Atlantic Division
('BOS', 'Celtics', 'Boston', 'Eastern', 'Atlantic'),
('BKN', 'Nets', 'Brooklyn', 'Eastern', 'Atlantic'), 
('NYK', 'Knicks', 'New York', 'Eastern', 'Atlantic'),
('PHI', '76ers', 'Philadelphia', 'Eastern', 'Atlantic'),
('TOR', 'Raptors', 'Toronto', 'Eastern', 'Atlantic'),

-- Eastern Conference - Central Division
('CHI', 'Bulls', 'Chicago', 'Eastern', 'Central'),
('CLE', 'Cavaliers', 'Cleveland', 'Eastern', 'Central'),
('DET', 'Pistons', 'Detroit', 'Eastern', 'Central'),
('IND', 'Pacers', 'Indiana', 'Eastern', 'Central'),
('MIL', 'Bucks', 'Milwaukee', 'Eastern', 'Central'),

-- Eastern Conference - Southeast Division
('ATL', 'Hawks', 'Atlanta', 'Eastern', 'Southeast'),
('CHA', 'Hornets', 'Charlotte', 'Eastern', 'Southeast'),
('MIA', 'Heat', 'Miami', 'Eastern', 'Southeast'),
('ORL', 'Magic', 'Orlando', 'Eastern', 'Southeast'),
('WAS', 'Wizards', 'Washington', 'Eastern', 'Southeast'),

-- Western Conference - Northwest Division  
('DEN', 'Nuggets', 'Denver', 'Western', 'Northwest'),
('MIN', 'Timberwolves', 'Minnesota', 'Western', 'Northwest'),
('OKC', 'Thunder', 'Oklahoma City', 'Western', 'Northwest'),
('POR', 'Trail Blazers', 'Portland', 'Western', 'Northwest'),
('UTA', 'Jazz', 'Utah', 'Western', 'Northwest'),

-- Western Conference - Pacific Division
('GSW', 'Warriors', 'Golden State', 'Western', 'Pacific'),
('LAC', 'Clippers', 'Los Angeles', 'Western', 'Pacific'),
('LAL', 'Lakers', 'Los Angeles', 'Western', 'Pacific'), 
('PHX', 'Suns', 'Phoenix', 'Western', 'Pacific'),
('SAC', 'Kings', 'Sacramento', 'Western', 'Pacific'),

-- Western Conference - Southwest Division
('DAL', 'Mavericks', 'Dallas', 'Western', 'Southwest'),
('HOU', 'Rockets', 'Houston', 'Western', 'Southwest'),
('MEM', 'Grizzlies', 'Memphis', 'Western', 'Southwest'),
('NOP', 'Pelicans', 'New Orleans', 'Western', 'Southwest'),
('SAS', 'Spurs', 'San Antonio', 'Western', 'Southwest')
ON CONFLICT (id) DO NOTHING;