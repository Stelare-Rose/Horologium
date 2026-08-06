CREATE TABLE IF NOT EXISTS Projects (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL CHECK (name != ''),
	state TEXT CHECK (state IN ('active', 'done')),
	color TEXT,
	completed_date TEXT,
	path TEXT,
	fingerprint INTEGER,
	CHECK (
		(state = 'done' AND completed_date IS NOT NULL) OR
		(state = 'active' AND completed_date IS NULL)
	)
);
