CREATE TABLE IF NOT EXISTS Quotas (
    id TEXT PRIMARY KEY,
    name TEXT,
    period TEXT NOT NULL CHECK (period IN ('day', 'week', 'month')),
    kind TEXT NOT NULL CHECK (kind IN ('min', 'max')),
    target_secs INTEGER NOT NULL CHECK (target_secs > 0),
    target_type TEXT NOT NULL CHECK (target_type IN ('tag', 'project')),
    target_id TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('active', 'inactive')),
    path TEXT,
    fingerprint INTEGER
);
