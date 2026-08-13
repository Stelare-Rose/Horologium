UPDATE Records
SET end = ?1
WHERE id = (SELECT id FROM Records WHERE start < ?1 ORDER BY start DESC LIMIT 1);
