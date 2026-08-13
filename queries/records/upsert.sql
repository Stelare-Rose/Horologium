INSERT INTO Records (id, name, status, start, project, path, fingerprint)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
ON CONFLICT (id) DO UPDATE SET
    name = excluded.name,
    status = excluded.status,
    start = excluded.start,
    project = excluded.project,
    path = excluded.path,
    fingerprint = excluded.fingerprint;
