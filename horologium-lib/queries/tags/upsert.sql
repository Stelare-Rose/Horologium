INSERT INTO Tags (id, name, color, path, fingerprint, content_hash)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(id) DO UPDATE SET
  name = excluded.name,
  color = excluded.color,
  path = excluded.path,
  fingerprint = excluded.fingerprint,
  content_hash = excluded.content_hash;
