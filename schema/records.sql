CREATE TABLE IF NOT EXISTS Records (
  id TEXT PRIMARY KEY,
  name TEXT,
	status TEXT CHECK (status IN ('active', 'inactive')),
	start INTEGER NOT NULL,
	end INTEGER,
	project TEXT REFERENCES Projects(id),
	path TEXT,
	fingerprint INTEGER
);
