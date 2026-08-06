CREATE TABLE IF NOT EXISTS Tags (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL CHECK (name != ''),
	color TEXT,
	path TEXT,
	fingerprint INTEGER
);
CREATE TABLE IF NOT EXISTS RecordsTags (
	tag_id TEXT NOT NULL REFERENCES Tags(id),
	record_id TEXT NOT NULL REFERENCES Records(id),
	position INTEGER NOT NULL,
	PRIMARY KEY (tag_id, record_id)
);
CREATE INDEX idx_recordstags_record ON RecordsTags(record_id);
