INSERT INTO Projects (id, name, state, color, completed_date, path, fingerprint)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
ON CONFLICT(id) DO UPDATE SET
  name = excluded.name,
  state = excluded.state,
  color = excluded.color,
  completed_date = excluded.completed_date,
  path = excluded.path,
  fingerprint = excluded.fingerprint;
