INSERT INTO Tags (id, name, color, path, fingerprint)
VALUES (?1, ?2, ?3, ?4, ?5)
ON CONFLICT(id) DO UPDATE SET
  name = excluded.name,
  color = excluded.color,
  path = excluded.path,
  fingerprint = excluded.fingerprint;
